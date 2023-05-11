#include "RXQueue.h"
#include "Adapter.h"
#include "netringiterator.h"
#include "Ringbuffer.h"
#include <net/virtualaddress.h>
#include <net/checksum.h>
#include "RXQueue.tmh"
#include "netiodef.h"

UINT8
GetLayer4Type(const VOID* buf, size_t len) {
	UINT8 ret = NetPacketLayer4TypeUnspecified;
	if (len < sizeof(IPV4_HEADER))
		return ret;

	IPV4_HEADER* ipv4hdr = (IPV4_HEADER*)buf;
	if (ipv4hdr->Version == IPV4_VERSION) {
		if (ipv4hdr->Protocol == IPPROTO_TCP)
			ret = NetPacketLayer4TypeTcp;
		else if (ipv4hdr->Protocol == IPPROTO_UDP)
			ret = NetPacketLayer4TypeUdp;
	}
	else if (ipv4hdr->Version == 6) {
		if (len < sizeof(IPV6_HEADER))
			return ret;

		IPV6_HEADER* ipv6hdr = (IPV6_HEADER*)buf;
		if (ipv6hdr->NextHeader == IPPROTO_TCP)
			ret = NetPacketLayer4TypeTcp;
		else if (ipv6hdr->NextHeader == IPPROTO_UDP)
			ret = NetPacketLayer4TypeUdp;
	}

	return ret;
}


NTSTATUS
ForTunAdapterCreateRXQueue(NETADAPTER NetAdapter, _Inout_ NETRXQUEUE_INIT* RxQueueInit) {

	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_ADAPTER, "%!FUNC! Entry");
	NTSTATUS status = STATUS_SUCCESS;
	NET_PACKET_QUEUE_CONFIG config;

	NET_PACKET_QUEUE_CONFIG_INIT(&config, ForTunRxQueueAdvance, ForTunRxQueueSetNotificationEnabled, ForTunRxQueueCancel);

	WDF_OBJECT_ATTRIBUTES attr;

	WDF_OBJECT_ATTRIBUTES_INIT_CONTEXT_TYPE(&attr, RXQUEUE_CONTEXT);
	NETPACKETQUEUE packetQueue;

	status = NetRxQueueCreate(RxQueueInit, &attr, &config, &packetQueue);
	if (!NT_SUCCESS(status)) {	
		goto done;
	}
	PADAPTER_CONTEXT adapterContext = AdapterGetContext(NetAdapter);
	adapterContext->RxQueue = packetQueue;
	PRXQUEUE_CONTEXT context = RxQueueGetContext(packetQueue);
	context->RingCollection = NetRxQueueGetRingCollection(packetQueue);
	context->NotificationEnabled = 0;
	context->AdapterContext = adapterContext;

	NET_EXTENSION_QUERY extension;
	NET_EXTENSION_QUERY_INIT(&extension, 
		NET_FRAGMENT_EXTENSION_VIRTUAL_ADDRESS_NAME,
		NET_FRAGMENT_EXTENSION_VIRTUAL_ADDRESS_VERSION_1,
		NetExtensionTypeFragment);
	NetRxQueueGetExtension(packetQueue, &extension, &context->VirtualAddressExtension);
	NET_EXTENSION_QUERY_INIT(&extension,
		NET_PACKET_EXTENSION_CHECKSUM_NAME, NET_PACKET_EXTENSION_CHECKSUM_VERSION_1, NetExtensionTypePacket);
	NetRxQueueGetExtension(packetQueue, &extension, &context->ChecksumExtension);

done:
	return status;
}

VOID
ForTunRxQueueAdvance(_In_ NETPACKETQUEUE PacketQueue)
{
	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_ADAPTER, "%!FUNC! Entry");
	PRXQUEUE_CONTEXT context = RxQueueGetContext(PacketQueue);
	PDEVICE_CONTEXT deviceContext = context->AdapterContext->DeviceContext;

	NET_RING_COLLECTION const* ringCollection = context->RingCollection;
	NET_RING_FRAGMENT_ITERATOR fi = NetRingGetAllFragments(ringCollection);
	NET_RING_PACKET_ITERATOR pi = NetRingGetAllPackets(ringCollection);
	NTSTATUS status;
	while (NetFragmentIteratorHasAny(&fi)) {
		WDFREQUEST request;
		status = WdfIoQueueRetrieveNextRequest(deviceContext->PendingWriteQueue, &request);
		if (!NT_SUCCESS(status)) {		
			break;
		}
		PVOID buffer;
		size_t bufferSize;
		status = WdfRequestRetrieveInputBuffer(request, sizeof(ULONG), &buffer, &bufferSize);
		if (!NT_SUCCESS(status)) {
			WdfRequestComplete(request, status);
			break;
		}

		NET_FRAGMENT* fragment = NetFragmentIteratorGetFragment(&fi);
		fragment->ValidLength = bufferSize;
		fragment->Offset = 0;
		PVOID* virtualAddress = NetExtensionGetFragmentVirtualAddressOffset(
			fragment,
			&context->VirtualAddressExtension,
			NetFragmentIteratorGetIndex(&fi)
		);
		RtlCopyMemory(virtualAddress, buffer, bufferSize);

		NET_PACKET* packet = NetPacketIteratorGetPacket(&pi);
		packet->FragmentCount = 1;
		packet->FragmentIndex = NetFragmentIteratorGetIndex(&fi);
		//packet->Layout = {0};
		NET_PACKET_LAYOUT layout = { 0 };
		packet->Layout = layout;

		NET_PACKET_CHECKSUM* checksum = NetExtensionGetPacketChecksum(&context->ChecksumExtension,NetPacketIteratorGetIndex(&pi));
		// Win11/2022 and newer
		if (checksum) {
			checksum->Layer3 = NetPacketRxChecksumEvaluationValid;
			checksum->Layer4 = NetPacketRxChecksumEvaluationInvalid;
			packet->Layout.Layer4Type = GetLayer4Type(virtualAddress, bufferSize);
		}
		NetFragmentIteratorAdvance(&fi);
		NetPacketIteratorAdvance(&pi);

		WdfRequestCompleteWithInformation(request, STATUS_SUCCESS, bufferSize);

	}
	NetFragmentIteratorSet(&fi);
	NetPacketIteratorSet(&pi);

}

VOID
ForTunRxQueueSetNotificationEnabled(
	_In_ NETPACKETQUEUE PacketQueue,
	_In_ BOOLEAN NotificationEnabled)
{
	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_ADAPTER, "Notification Enabled:%d", NotificationEnabled);
	PRXQUEUE_CONTEXT context = RxQueueGetContext(PacketQueue);
	InterlockedExchangeNoFence(&context->NotificationEnabled, NotificationEnabled);
}

VOID
ForTunRxQueueCancel(_In_ NETPACKETQUEUE PacketQueue)
{
	PRXQUEUE_CONTEXT context = RxQueueGetContext(PacketQueue);
	NET_RING_COLLECTION const* ringCollection = context->RingCollection;
	NET_RING* packetRing = ringCollection->Rings[NetRingTypePacket];
	NET_RING* fragmentRing = ringCollection->Rings[NetRingTypeFragment];
	UINT32 currentPacketIndex = packetRing->BeginIndex;
	UINT32 packetEndIndex = packetRing->EndIndex;

	while (currentPacketIndex != packetEndIndex) {
		NET_PACKET* packet = NetRingGetPacketAtIndex(packetRing, currentPacketIndex);
		packet->Ignore = 1;
		currentPacketIndex = NetRingIncrementIndex(packetRing, currentPacketIndex);
	}

	packetRing->BeginIndex = packetRing->EndIndex;

	fragmentRing->BeginIndex = fragmentRing->EndIndex;
}