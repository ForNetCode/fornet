#pragma once


#include "precomp.h"
#include "Device.h"

EXTERN_C_START

typedef struct _ADAPTER_CONTEXT {
	NETPACKETQUEUE RxQueue;
	PDEVICE_CONTEXT DeviceContext;
} ADAPTER_CONTEXT, *PADAPTER_CONTEXT;


WDF_DECLARE_CONTEXT_TYPE_WITH_NAME(ADAPTER_CONTEXT, AdapterGetContext)



NTSTATUS
ForTunAdapterCreate(PDEVICE_CONTEXT DeviceContext, WDFDEVICE Device);

NTSTATUS ForTunAdapterNotifyRx(NETADAPTER NetAdapter);

VOID ForTunAdapterSetLinkState(NETADAPTER Adapter, NET_IF_MEDIA_CONNECT_STATE State);

EXTERN_C_END