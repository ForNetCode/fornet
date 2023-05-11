/*++

Module Name:

    device.h

Abstract:

    This file contains the device definitions.

Environment:

    Kernel-mode Driver Framework

--*/
#pragma once
#include "precomp.h"
#include "Ringbuffer.h"

EXTERN_C_START

//
// The device context performs the same job as
// a WDM device extension in the driver frameworks
//
typedef struct _DEVICE_CONTEXT
{

    WDFQUEUE PendingWriteQueue;
    WDFQUEUE PendingReadQueue;

    PPOOL_QUEUE PoolQueue;
    WDFSPINLOCK readLock;

    NETADAPTER Adapter;
} DEVICE_CONTEXT, *PDEVICE_CONTEXT;

//
// This macro will generate an inline function called DeviceGetContext
// which will be used to get a pointer to the device context memory
// in a type safe manner.
//
WDF_DECLARE_CONTEXT_TYPE_WITH_NAME(DEVICE_CONTEXT, DeviceGetContext)

//
// Function to initialize the device and its callbacks
//
NTSTATUS
ForTunCreateDevice(
    _Inout_ PWDFDEVICE_INIT DeviceInit
    );


EVT_WDF_DEVICE_CONTEXT_CLEANUP ForTunDeviceCleanUp;

EVT_WDF_FILE_CLEANUP ForTunFileCleanUp;

EXTERN_C_END
