#include "TXQueue.h"
#include "Device.h"
#include "netringiterator.h"
#include <net/virtualaddress.h>
#include "TXQueue.tmh"



NTSTATUS
ForTunAdapterCreateTXQueue(_In_ NETADAPTER NetAdapter, _Inout_ NETTXQUEUE_INIT* TxQueueInit) {
	NTSTATUS status = STATUS_SUCCESS;

	NET_PACKET_QUEUE_CONFIG config;
	NETPACKETQUEUE queue;
	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_ADAPTER, "%!FUNC! Entry");
	NET_PACKET_QUEUE_CONFIG_INIT(&config, ForTunTxQueueAdvance, ForTunTxQueueSetNotificationEnabled, ForTunTxQueueCancel);
	
	WDF_OBJECT_ATTRIBUTES attr;
	WDF_OBJECT_ATTRIBUTES_INIT_CONTEXT_TYPE(&attr, TXQUEUE_CONTEXT);

	status = NetTxQueueCreate(TxQueueInit, &attr, &config, &queue);
	if (!NT_SUCCESS(status)) {
		goto done;
	}
	PTXQUEUE_CONTEXT context = TxQueueGetContext(queue);
	context->RingCollection = NetTxQueueGetRingCollection(queue);
	context->AdapterContext = AdapterGetContext(NetAdapter);

	NET_EXTENSION_QUERY extension;
	NET_EXTENSION_QUERY_INIT(&extension,
		NET_FRAGMENT_EXTENSION_VIRTUAL_ADDRESS_NAME,
		NET_FRAGMENT_EXTENSION_VIRTUAL_ADDRESS_VERSION_1,
		NetExtensionTypeFragment);
	NetTxQueueGetExtension(queue, &extension, &context->VirtualAddressExtension);

done:
	return status;
}

VOID
ForTunTxQueueSetNotificationEnabled(_In_ NETPACKETQUEUE PacketQueue, _In_ BOOLEAN NotificationEnabled)
{
	UNREFERENCED_PARAMETER(PacketQueue);
	UNREFERENCED_PARAMETER(NotificationEnabled);
}

VOID
ForTunTxQueueAdvance(_In_ NETPACKETQUEUE PacketQueue)
{	
	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_ADAPTER, "%!FUNC! Entry");

	PTXQUEUE_CONTEXT context = TxQueueGetContext(PacketQueue);
	PDEVICE_CONTEXT deviceContext = context->AdapterContext->DeviceContext;

	NET_RING_COLLECTION const* ringCollection = context->RingCollection;
	NET_RING_PACKET_ITERATOR pi = NetRingGetAllPackets(ringCollection);

	NTSTATUS status = STATUS_SUCCESS;
	// in
	while (NetPacketIteratorHasAny(&pi)) {
		NET_PACKET* packet = NetPacketIteratorGetPacket(&pi);
		if (!packet->Ignore && !packet->Scratch) {
			// Process
			NET_RING_FRAGMENT_ITERATOR fi = NetPacketIteratorGetFragments(&pi);
			WDFREQUEST request;
			WdfSpinLockAcquire(deviceContext->readLock);
			status = WdfIoQueueRetrieveNextRequest(deviceContext->PendingReadQueue, &request);
			UCHAR* buffer = NULL;

			PPOOL_QUEUE_ITEM poolQueueItem = NULL;

			if (NT_SUCCESS(status)) {
				status = WdfRequestRetrieveOutputBuffer(request, sizeof(LONG), &buffer, NULL);
				if (!NT_SUCCESS(status)) {					
					//This should never reach
					TraceEvents(TRACE_LEVEL_ERROR, TRACE_ADAPTER, "wdf request retrieve buffer fail: %d", status);
					WdfRequestComplete(request, status);
				}
			}
			else {
				poolQueueItem = PoolQueueGetFromPool(deviceContext->PoolQueue);
				if (!poolQueueItem) {
					TraceEvents(TRACE_LEVEL_ERROR, TRACE_ADAPTER, "PoolQueue is full");
					status = STATUS_INSUFFICIENT_RESOURCES;
				}
				else {
					status = STATUS_SUCCESS;
				}
			}
			
			if (!NT_SUCCESS(status)) {
				WdfSpinLockRelease(deviceContext->readLock);
				NetFragmentIteratorSet(&fi);
			}
			else {			
				SIZE_T length = 0;
				while (NetFragmentIteratorHasAny(&fi)) {
					NET_FRAGMENT* fragment = NetFragmentIteratorGetFragment(&fi);
					BYTE* netBuf = (BYTE *)NetExtensionGetFragmentVirtualAddressOffset(fragment, &context->VirtualAddressExtension, NetFragmentIteratorGetIndex(&fi));
					if (buffer) {
						RtlCopyMemory(buffer + length, netBuf, fragment->ValidLength);
					}
					else if(poolQueueItem) {
						RtlCopyMemory(poolQueueItem->Data + length, netBuf, fragment->ValidLength);
					}
					UINT64 ValidLength = fragment->ValidLength;
					length = length + (SIZE_T)ValidLength;
					NetFragmentIteratorAdvance(&fi);
				}
				if (buffer) {
					WdfRequestCompleteWithInformation(request, status, length);
				}
				else if(poolQueueItem){
					poolQueueItem->DataSize = length;
					PoolQueuePutToQueue(deviceContext->PoolQueue, poolQueueItem);
				}
				WdfSpinLockRelease(deviceContext->readLock);
			}

			/*
			while (NetFragmentIteratorHasAny(&fi)) {
				NET_FRAGMENT* fragment = NetFragmentIteratorGetFragment(&fi);
				NET_FRAGMENT_VIRTUAL_ADDRESS* virtualAddr = NetExtensionGetFragmentVirtualAddress(
					&context->VirtualAddressExtension, NetFragmentIteratorGetIndex(&fi));
				if (fragment->ValidLength > 0) {
					WDFREQUEST request;
					WdfSpinLockAcquire(deviceContext->readLock);
					status = WdfIoQueueRetrieveNextRequest(deviceContext->PendingReadQueue, &request);
					if (NT_SUCCESS(status)) {
						WdfSpinLockRelease(deviceContext->readLock);
						PVOID buffer;
						status = WdfRequestRetrieveOutputBuffer(request, sizeof(LONG), &buffer, NULL);
						if (!NT_SUCCESS(status)) {
							WdfRequestComplete(request, status);
						}
						else {
							RtlCopyMemory(buffer, virtualAddr, fragment->ValidLength);
							WdfRequestCompleteWithInformation(request, status, fragment->ValidLength);
						}
					}
					else {
						PPOOL_QUEUE_ITEM poolQueueItem = PoolQueueGetFromPool(deviceContext->PoolQueue);
						if (!poolQueueItem) {
							TraceEvents(TRACE_LEVEL_ERROR, TRACE_ADAPTER, "PoolQueue is full");
						}
						else {
							RtlCopyMemory(&poolQueueItem->Data, virtualAddr, fragment->ValidLength);
							poolQueueItem->DataSize = fragment->ValidLength;
							PoolQueuePutToQueue(deviceContext->PoolQueue, poolQueueItem);
						}

						WdfSpinLockRelease(deviceContext->readLock);
					}
				}
				NetFragmentIteratorAdvance(&fi);

			} */
			NET_PACKET* p = NetPacketIteratorGetPacket(&pi);
			NET_RING* const fragmentRing = NetRingCollectionGetFragmentRing(fi.Iterator.Rings);
			UINT32 const lastFragmentIndex = NetRingAdvanceIndex(fragmentRing, p->FragmentIndex, p->FragmentCount);
			fragmentRing->BeginIndex = lastFragmentIndex;

			//Process
		}
		NetPacketIteratorAdvance(&pi);
		if (!NT_SUCCESS(status)) {
			break;
		}
	}
	NetPacketIteratorSet(&pi);
}

VOID
ForTunTxQueueCancel(_In_ NETPACKETQUEUE PacketQueue)
{
	PTXQUEUE_CONTEXT queue = TxQueueGetContext(PacketQueue);
	NET_RING_COLLECTION const* ringCollection = queue->RingCollection;
	NET_RING* packetRing = ringCollection->Rings[NetRingTypePacket];
	UINT32 currentPacketIndex = packetRing->BeginIndex;
	UINT32 packetEndIndex = packetRing->EndIndex;
	while (currentPacketIndex != packetEndIndex) {
		NET_PACKET* packet = NetRingGetPacketAtIndex(packetRing, currentPacketIndex);
		packet->Scratch = 1;
		currentPacketIndex = NetRingIncrementIndex(packetRing, currentPacketIndex);
	}
	packetRing->BeginIndex = packetRing->EndIndex;

	NET_RING* fragmentRing = NetRingCollectionGetFragmentRing(ringCollection);
	fragmentRing->BeginIndex = fragmentRing->EndIndex;
}