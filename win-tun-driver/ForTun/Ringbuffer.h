#pragma once
#include "precomp.h"


typedef struct _POOL_QUEUE{
    LIST_ENTRY PoolHead;
    LIST_ENTRY QueueHead;
} POOL_QUEUE, *PPOOL_QUEUE;


typedef struct _POOL_QUEUE_ITEM {
    LIST_ENTRY PoolEntry;
    LIST_ENTRY QueueEntry;
    size_t DataSize;
    UCHAR Data[0xffff];

}POOL_QUEUE_ITEM, *PPOOL_QUEUE_ITEM;

NTSTATUS
PoolQueueCreate(PPOOL_QUEUE* PoolQueue);

PPOOL_QUEUE_ITEM
PoolQueueGetFromPool(PPOOL_QUEUE PoolQueue);

VOID
PoolQueuePutToPool(PPOOL_QUEUE PoolQueue, PPOOL_QUEUE_ITEM Item);

VOID
PoolQueuePutToQueue(PPOOL_QUEUE PoolQueue, PPOOL_QUEUE_ITEM Item);


PPOOL_QUEUE_ITEM
PoolQueueGetFromQueue(PPOOL_QUEUE PoolQueue);


VOID
PoolQueueFree(PPOOL_QUEUE PoolQueue);

