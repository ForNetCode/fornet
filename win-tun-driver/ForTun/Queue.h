/*++

Module Name:

    queue.h

Abstract:

    This file contains the queue definitions.

Environment:

    Kernel-mode Driver Framework

--*/
#pragma once
#include "precomp.h"

EXTERN_C_START

NTSTATUS
ForTunQueueInitialize(
    _In_ WDFDEVICE Device
    );

//
// Events from the IoQueue object
//
EVT_WDF_IO_QUEUE_IO_DEVICE_CONTROL ForTunEvtIoDeviceControl;
EVT_WDF_IO_QUEUE_IO_STOP ForTunEvtIoStop;
EVT_WDF_IO_QUEUE_IO_READ ForTunEvtIoRead;
EVT_WDF_IO_QUEUE_IO_WRITE ForTunEvtIoWrite;

EXTERN_C_END
