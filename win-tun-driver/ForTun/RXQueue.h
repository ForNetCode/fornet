#pragma once
#include "precomp.h"
#include "Adapter.h"

EXTERN_C_START

EVT_NET_ADAPTER_CREATE_RXQUEUE ForTunAdapterCreateRXQueue;

typedef struct _RXQUEUE_CONTEXT {
	NET_RING_COLLECTION const* RingCollection;
	_Interlocked_ LONG NotificationEnabled;

	PADAPTER_CONTEXT AdapterContext;
	NET_EXTENSION VirtualAddressExtension;
	NET_EXTENSION ChecksumExtension;

} RXQUEUE_CONTEXT, *PRXQUEUE_CONTEXT;


WDF_DECLARE_CONTEXT_TYPE_WITH_NAME(RXQUEUE_CONTEXT, RxQueueGetContext)

EXTERN_C_END

EVT_PACKET_QUEUE_SET_NOTIFICATION_ENABLED ForTunRxQueueSetNotificationEnabled;
EVT_PACKET_QUEUE_ADVANCE ForTunRxQueueAdvance;
EVT_PACKET_QUEUE_CANCEL ForTunRxQueueCancel;
