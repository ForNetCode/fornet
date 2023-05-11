#include "ringbuffer.h"

NTSTATUS
PoolQueueCreate(PPOOL_QUEUE* PoolQueue) {
    PPOOL_QUEUE poolQueue = (PPOOL_QUEUE)ExAllocatePool2(POOL_FLAG_NON_PAGED, sizeof(POOL_QUEUE),'pque');
    if (!poolQueue) {
        return STATUS_MEMORY_NOT_ALLOCATED;
    }
    InitializeListHead(&poolQueue->PoolHead);
    InitializeListHead(&poolQueue->QueueHead);

    *PoolQueue = poolQueue;
    return STATUS_SUCCESS;
}


PPOOL_QUEUE_ITEM
PoolQueueGetFromPool(PPOOL_QUEUE PoolQueue) {
    if (IsListEmpty(&PoolQueue->PoolHead)) {
        return (PPOOL_QUEUE_ITEM)ExAllocatePool2(POOL_FLAG_NON_PAGED, sizeof(POOL_QUEUE_ITEM), 'pque');
    }
    else {
        return CONTAINING_RECORD(RemoveHeadList(&PoolQueue->PoolHead), POOL_QUEUE_ITEM, PoolEntry);
    }
}

VOID
PoolQueuePutToPool(PPOOL_QUEUE PoolQueue, PPOOL_QUEUE_ITEM Item) {
    InsertTailList(&PoolQueue->PoolHead, &Item->PoolEntry);
}

VOID
PoolQueuePutToQueue(PPOOL_QUEUE PoolQueue, PPOOL_QUEUE_ITEM Item) {
    InsertTailList(&PoolQueue->QueueHead, &Item->QueueEntry);
}


PPOOL_QUEUE_ITEM
PoolQueueGetFromQueue(PPOOL_QUEUE PoolQueue) {
    BOOLEAN isEmpty = IsListEmpty(&PoolQueue->QueueHead);
    if (isEmpty) {
        return  NULL;
    }
    else {
        PLIST_ENTRY entry = RemoveHeadList(&PoolQueue->QueueHead);
        return CONTAINING_RECORD(entry, POOL_QUEUE_ITEM, QueueEntry);
    }
}


VOID
PoolQueueFree(PPOOL_QUEUE PoolQueue) {
    if (PoolQueue == NULL) {
        return;
    }


   while (!IsListEmpty(&PoolQueue->PoolHead)) {
       PLIST_ENTRY entry = RemoveHeadList(&PoolQueue->PoolHead);
	   PPOOL_QUEUE_ITEM item = CONTAINING_RECORD(entry, POOL_QUEUE_ITEM, PoolEntry);
       ExFreePoolWithTag(item, 'pque');   
   }
   while (!IsListEmpty(&PoolQueue->QueueHead)) {
       PLIST_ENTRY entry = RemoveHeadList(&PoolQueue->QueueHead);
       PPOOL_QUEUE_ITEM item = CONTAINING_RECORD(entry, POOL_QUEUE_ITEM, QueueEntry);
       ExFreePoolWithTag(item, 'pque');

   }   

   ExFreePoolWithTag(PoolQueue, 'pque');
}

