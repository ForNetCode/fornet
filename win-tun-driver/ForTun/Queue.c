/*++

Module Name:

    queue.c

Abstract:

    This file contains the queue entry points and callbacks.

Environment:

    Kernel-mode Driver Framework

--*/

#include "Queue.h"
#include "Device.h"
#include "Ringbuffer.h"
#include "Adapter.h"
#include "Public.h"
#include "queue.tmh"

#ifdef ALLOC_PRAGMA
#pragma alloc_text (PAGE, ForTunQueueInitialize)
#endif


NTSTATUS
ForTunQueueInitialize(
    _In_ WDFDEVICE Device
    )
/*++

Routine Description:

     The I/O dispatch callbacks for the frameworks device object
     are configured in this function.

     A single default I/O Queue is configured for parallel request
     processing, and a driver context memory allocation is created
     to hold our structure QUEUE_CONTEXT.

Arguments:

    Device - Handle to a framework device object.

Return Value:

    VOID

--*/
{
    WDFQUEUE queue;
    NTSTATUS status;
    WDF_IO_QUEUE_CONFIG queueConfig;

    PAGED_CODE();

    TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_QUEUE, "%!FUNC! Entry");

    //
    // Configure a default queue so that requests that are not
    // configure-fowarded using WdfDeviceConfigureRequestDispatching to goto
    // other queues get dispatched here.
    //
    WDF_IO_QUEUE_CONFIG_INIT_DEFAULT_QUEUE(
         &queueConfig,
         WdfIoQueueDispatchSequential
        );

    queueConfig.EvtIoDeviceControl = ForTunEvtIoDeviceControl;
    queueConfig.EvtIoRead = ForTunEvtIoRead;
    queueConfig.EvtIoStop = ForTunEvtIoStop;
    queueConfig.EvtIoWrite = ForTunEvtIoWrite;

    status = WdfIoQueueCreate(
                 Device,
                 &queueConfig,
                 WDF_NO_OBJECT_ATTRIBUTES,
                 &queue
                 );

    if(!NT_SUCCESS(status)) {
        TraceEvents(TRACE_LEVEL_ERROR, TRACE_QUEUE, "create default queue failed %!STATUS!", status);
        return status;
    }
    
    WDF_IO_QUEUE_CONFIG_INIT(&queueConfig, WdfIoQueueDispatchManual);

    WDFQUEUE readQueue;
    status = WdfIoQueueCreate(Device, &queueConfig, WDF_NO_OBJECT_ATTRIBUTES, &readQueue);
    if (!NT_SUCCESS(status)) {
        TraceEvents(TRACE_LEVEL_ERROR, TRACE_QUEUE, "create read queue failed %!STATUS!", status);
        return status;
    }
    //status = WdfDeviceConfigureRequestDispatching(Device, readQueue, WdfRequestTypeRead);
    //if (!NT_SUCCESS(status)) {    
    //    goto done;
    //}
    WDFQUEUE writeQueue;
    WDF_IO_QUEUE_CONFIG_INIT(&queueConfig, WdfIoQueueDispatchManual);
    status = WdfIoQueueCreate(Device, &queueConfig, WDF_NO_OBJECT_ATTRIBUTES, &writeQueue);

    if (!NT_SUCCESS(status)) {
        TraceEvents(TRACE_LEVEL_ERROR, TRACE_QUEUE, "create write queue failed %!STATUS!", status);
        return status;
    }
    //status = WdfDeviceConfigureRequestDispatching(Device, writeQueue, WdfRequestTypeWrite);
    PDEVICE_CONTEXT device_context = DeviceGetContext(Device);

    /*
    WDF_OBJECT_ATTRIBUTES attr;
    WDF_OBJECT_ATTRIBUTES_INIT(&attr);
    attr.ParentObject = Device;
    WDFMEMORY BufferHandler;
    status = WdfMemoryCreate(&attr, NonPagedPool, 'FTun', READ_POOL_SIZE, &BufferHandler, &device_context->PReadBuffer);
    if (!NT_SUCCESS(status)) {
        TraceEvents(TRACE_LEVEL_ERROR, TRACE_QUEUE, "create ReadBuffer failed %!STATUS!", status);
        goto done;
    }
    RtlZeroMemory(device_context->PReadBuffer, READ_POOL_SIZE);
    RingBufferInitialize(&device_context->ReadRingBuffer, device_context->PReadBuffer, READ_POOL_SIZE);
     */
    PPOOL_QUEUE poolQueue;
    status = PoolQueueCreate(&poolQueue);
    if (!NT_SUCCESS(status)) {
        TraceEvents(TRACE_LEVEL_ERROR, TRACE_QUEUE, "create PoolQueue failed %!STATUS!", status);
        goto done;
    }
    device_context->PoolQueue = poolQueue;
    device_context->PendingReadQueue = readQueue;
    device_context->PendingWriteQueue = writeQueue;
    

    WDF_OBJECT_ATTRIBUTES attr;
    WDF_OBJECT_ATTRIBUTES_INIT(&attr);
    attr.ParentObject = Device;
    
    WdfSpinLockCreate(&attr, &device_context->readLock);

    
done:
    return status;
}

VOID
ForTunEvtIoDeviceControl(
    _In_ WDFQUEUE Queue,
    _In_ WDFREQUEST Request,
    _In_ size_t OutputBufferLength,
    _In_ size_t InputBufferLength,
    _In_ ULONG IoControlCode
    )
/*++

Routine Description:

    This event is invoked when the framework receives IRP_MJ_DEVICE_CONTROL request.

Arguments:

    Queue -  Handle to the framework queue object that is associated with the
             I/O request.

    Request - Handle to a framework request object.

    OutputBufferLength - Size of the output buffer in bytes

    InputBufferLength - Size of the input buffer in bytes

    IoControlCode - I/O control code.

Return Value:

    VOID

--*/
{
    TraceEvents(TRACE_LEVEL_INFORMATION, 
                TRACE_QUEUE, 
                "%!FUNC! Queue 0x%p, Request 0x%p OutputBufferLength %d InputBufferLength %d IoControlCode %d", 
                Queue, Request, (int) OutputBufferLength, (int) InputBufferLength, IoControlCode);
    
    switch (IoControlCode) {
    case FOR_TUN_IOCTL_OPEN_ADAPTER:     
        NETADAPTER adapter = DeviceGetContext(WdfIoQueueGetDevice(Queue))->Adapter;
        ForTunAdapterSetLinkState(adapter, MediaConnectStateConnected);
        WdfRequestComplete(Request, STATUS_SUCCESS);
        break;

    default:
        WdfRequestComplete(Request, STATUS_INVALID_DEVICE_REQUEST);
    }
}

VOID
ForTunEvtIoStop(
    _In_ WDFQUEUE Queue,
    _In_ WDFREQUEST Request,
    _In_ ULONG ActionFlags
)
/*++

Routine Description:

    This event is invoked for a power-managed queue before the device leaves the working state (D0).

Arguments:

    Queue -  Handle to the framework queue object that is associated with the
             I/O request.

    Request - Handle to a framework request object.

    ActionFlags - A bitwise OR of one or more WDF_REQUEST_STOP_ACTION_FLAGS-typed flags
                  that identify the reason that the callback function is being called
                  and whether the request is cancelable.

Return Value:

    VOID

--*/
{
    TraceEvents(TRACE_LEVEL_INFORMATION, 
                TRACE_QUEUE, 
                "%!FUNC! Queue 0x%p, Request 0x%p ActionFlags %d, trigger IoStop", 
                Queue, Request, ActionFlags);

    //
    // In most cases, the EvtIoStop callback function completes, cancels, or postpones
    // further processing of the I/O request.
    //
    // Typically, the driver uses the following rules:
    //
    // - If the driver owns the I/O request, it calls WdfRequestUnmarkCancelable
    //   (if the request is cancelable) and either calls WdfRequestStopAcknowledge
    //   with a Requeue value of TRUE, or it calls WdfRequestComplete with a
    //   completion status value of STATUS_SUCCESS or STATUS_CANCELLED.
    //
    //   Before it can call these methods safely, the driver must make sure that
    //   its implementation of EvtIoStop has exclusive access to the request.
    //
    //   In order to do that, the driver must synchronize access to the request
    //   to prevent other threads from manipulating the request concurrently.
    //   The synchronization method you choose will depend on your driver's design.
    //
    //   For example, if the request is held in a shared context, the EvtIoStop callback
    //   might acquire an internal driver lock, take the request from the shared context,
    //   and then release the lock. At this point, the EvtIoStop callback owns the request
    //   and can safely complete or requeue the request.
    //
    // - If the driver has forwarded the I/O request to an I/O target, it either calls
    //   WdfRequestCancelSentRequest to attempt to cancel the request, or it postpones
    //   further processing of the request and calls WdfRequestStopAcknowledge with
    //   a Requeue value of FALSE.
    //
    // A driver might choose to take no action in EvtIoStop for requests that are
    // guaranteed to complete in a small amount of time.
    //
    // In this case, the framework waits until the specified request is complete
    // before moving the device (or system) to a lower power state or removing the device.
    // Potentially, this inaction can prevent a system from entering its hibernation state
    // or another low system power state. In extreme cases, it can cause the system
    // to crash with bugcheck code 9F.
    //

    return;
}


VOID
ForTunEvtIoRead(
    IN WDFQUEUE Queue,
    IN WDFREQUEST Request,
    IN size_t Length
)
{
    UNREFERENCED_PARAMETER(Length);
    NTSTATUS status;

    PDEVICE_CONTEXT deviceContext = DeviceGetContext(WdfIoQueueGetDevice(Queue));
    
    PVOID readBuffer;
    status = WdfRequestRetrieveOutputBuffer(Request, sizeof(LONG), &readBuffer, NULL);
    if (!NT_SUCCESS(status)) {
        goto logErr;

    }

    WdfSpinLockAcquire(deviceContext->readLock);

    PPOOL_QUEUE_ITEM poolQueueItem = PoolQueueGetFromQueue(deviceContext->PoolQueue);

    if (poolQueueItem) {
        RtlCopyMemory(readBuffer, &poolQueueItem->Data, poolQueueItem->DataSize);
        size_t dataSize = poolQueueItem->DataSize;
        PoolQueuePutToPool(deviceContext->PoolQueue, poolQueueItem);
        WdfSpinLockRelease(deviceContext->readLock);
        WdfRequestCompleteWithInformation(Request, STATUS_SUCCESS, dataSize);
    }
/*
    status = RingBufferRead(&deviceContext->ReadRingBuffer, (BYTE*)&readBuffer, Length, &bytesCopied);
    if (!NT_SUCCESS(status)) {
        WdfSpinLockRelease(deviceContext->readLock);
        goto logErr;
    }
    if (bytesCopied > 0) {
        WdfSpinLockRelease(deviceContext->readLock);
        WdfRequestCompleteWithInformation(Request, status, bytesCopied);
    }*/
    else {    
        status = WdfRequestForwardToIoQueue(Request, deviceContext->PendingReadQueue);
        WdfSpinLockRelease(deviceContext->readLock);
        if (!NT_SUCCESS(status)) {
            goto logErr;
        }
    }    

    return;
logErr:
    WdfRequestComplete(Request, status);
    TraceEvents(TRACE_LEVEL_ERROR, TRACE_QUEUE, "read occur error:%d", status);
}

VOID
ForTunEvtIoWrite(
    IN WDFQUEUE Queue,
    IN WDFREQUEST Request,
    IN size_t Length
)
{
    UNREFERENCED_PARAMETER(Length);

    PDEVICE_CONTEXT context = DeviceGetContext(WdfIoQueueGetDevice(Queue));

    WDFQUEUE PendingQueue = context->PendingWriteQueue;

    NTSTATUS status = WdfRequestForwardToIoQueue(Request, PendingQueue);
    if (!NT_SUCCESS(status)) {
        WdfRequestComplete(Request, status);
        goto logErr;
    }

    //this should always be success
    ForTunAdapterNotifyRx(context->Adapter);

    return;
logErr:
    TraceEvents(TRACE_LEVEL_ERROR, TRACE_QUEUE, "write occur error:%d", status);
}