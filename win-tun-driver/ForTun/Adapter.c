#include "Adapter.h"
#include "rxqueue.h"
#include "txqueue.h"
#include "Adapter.tmh"

#define FOR_TUN_MEDIA_MAX_SPEED 1'000'000'000

#if (NETADAPTER_VERSION_MAJOR>=2) && (NETADAPTER_VERSION_MINOR>=1)
EVT_NET_ADAPTER_OFFLOAD_SET_RX_CHECKSUM AdapterOffloadSetRxCHecksum;


VOID
AdapterOffloadSetRxChecksum(NETADAPTER Adapter, NETOFFLOAD offload) {
    UNREFERENCED_PARAMETER(Adapter);
    UNREFERENCED_PARAMETER(offload);
}
#endif


NTSTATUS
ForTunAdapterCreate(PDEVICE_CONTEXT DeviceContext, WDFDEVICE Device) {

	NTSTATUS status = STATUS_SUCCESS;

    TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_ADAPTER, "%!FUNC! Entry");

    NETADAPTER_INIT* adapterInit = NetAdapterInitAllocate(Device);

    NET_ADAPTER_DATAPATH_CALLBACKS datapathCallbacks;
    NET_ADAPTER_DATAPATH_CALLBACKS_INIT(&datapathCallbacks,
        ForTunAdapterCreateTXQueue,
        ForTunAdapterCreateRXQueue);
    NetAdapterInitSetDatapathCallbacks(adapterInit, &datapathCallbacks);

    WDF_OBJECT_ATTRIBUTES adapterAttributes;
    WDF_OBJECT_ATTRIBUTES_INIT_CONTEXT_TYPE(&adapterAttributes, ADAPTER_CONTEXT);

    NETADAPTER netAdapter;
    status = NetAdapterCreate(adapterInit, &adapterAttributes, &netAdapter);
    if (!NT_SUCCESS(status)) {
        goto adapterFail;
	}

    NetAdapterInitFree(adapterInit);
    adapterInit = NULL;


    DeviceContext->Adapter = netAdapter;
    PADAPTER_CONTEXT context = AdapterGetContext(netAdapter);
    context->DeviceContext = DeviceContext;

    // Capabilities
    // Limit
    NET_ADAPTER_RX_CAPABILITIES rxCapabilities;
    NET_ADAPTER_RX_CAPABILITIES_INIT_SYSTEM_MANAGED(&rxCapabilities, 65536,1);
    NET_ADAPTER_TX_CAPABILITIES txCapabilities;
    NET_ADAPTER_TX_CAPABILITIES_INIT(&txCapabilities, 1);
    NetAdapterSetDataPathCapabilities(netAdapter, &txCapabilities, &rxCapabilities);

    //linkLayer
    NET_ADAPTER_LINK_LAYER_CAPABILITIES linkLayerCapabilities;
    NET_ADAPTER_LINK_LAYER_CAPABILITIES_INIT(&linkLayerCapabilities, FOR_TUN_MEDIA_MAX_SPEED, FOR_TUN_MEDIA_MAX_SPEED);
    NetAdapterSetLinkLayerCapabilities(netAdapter, &linkLayerCapabilities);
    NetAdapterSetLinkLayerMtuSize(netAdapter, 0xFFFF);

    //LinkState
    ForTunAdapterSetLinkState(netAdapter, MediaConnectStateDisconnected);

#if(NETADAPTER_VERSION_MAJOR>=2) && (NETADAPTER_VERSION_MINOR>=1)
    AdapterOffloadSetRxChecksum(netAdapter);
#endif

    status = NetAdapterStart(netAdapter);
    
    goto done;

adapterFail:
    NetAdapterInitFree(adapterInit);
    adapterInit = NULL;
done:
    return status;
}


NTSTATUS ForTunAdapterNotifyRx(NETADAPTER NetAdapter)
{
    if (NetAdapter == WDF_NO_HANDLE) {
        TraceEvents(TRACE_LEVEL_ERROR, TRACE_ADAPTER, "Adapter no initialized");
        return STATUS_DEVICE_NOT_READY;
    }

    NETPACKETQUEUE rxQueue = AdapterGetContext(NetAdapter)->RxQueue;
    PRXQUEUE_CONTEXT rxContext = RxQueueGetContext(rxQueue);
    if (InterlockedExchange(&rxContext->NotificationEnabled, FALSE) == TRUE) {
        TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_ADAPTER, "begin to notify rxQueue");
        NetRxQueueNotifyMoreReceivedPacketsAvailable(rxQueue);
    }

    return STATUS_SUCCESS;
}

VOID ForTunAdapterSetLinkState(NETADAPTER Adapter, NET_IF_MEDIA_CONNECT_STATE State)
{
    NET_ADAPTER_LINK_STATE linkState;
    NET_ADAPTER_LINK_STATE_INIT(&linkState,
        FOR_TUN_MEDIA_MAX_SPEED,
        State,
        MediaDuplexStateFull,
        NetAdapterPauseFunctionTypeUnsupported,
        NetAdapterAutoNegotiationFlagNone);
    NetAdapterSetLinkState(Adapter, &linkState);
    
  
}
