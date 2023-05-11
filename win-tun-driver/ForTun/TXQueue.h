#pragma once
#include "precomp.h"
#include "Adapter.h"

EXTERN_C_START


EVT_NET_ADAPTER_CREATE_TXQUEUE ForTunAdapterCreateTXQueue;


EXTERN_C_END

typedef struct _TXQUEUE_CONTEXT {
	NET_RING_COLLECTION const* RingCollection;
	PADAPTER_CONTEXT AdapterContext;
	NET_EXTENSION VirtualAddressExtension;
} TXQUEUE_CONTEXT, *PTXQUEUE_CONTEXT;


WDF_DECLARE_CONTEXT_TYPE_WITH_NAME(TXQUEUE_CONTEXT, TxQueueGetContext)

EVT_PACKET_QUEUE_SET_NOTIFICATION_ENABLED ForTunTxQueueSetNotificationEnabled;
EVT_PACKET_QUEUE_ADVANCE ForTunTxQueueAdvance;
EVT_PACKET_QUEUE_CANCEL ForTunTxQueueCancel;