/*++

Module Name:

    driver.h

Abstract:

    This file contains the driver definitions.

Environment:

    Kernel-mode Driver Framework

--*/

#include "precomp.h"

#include "device.h"
#include "queue.h"

EXTERN_C_START

//
// WDFDRIVER Events
//

DRIVER_INITIALIZE DriverEntry;
EVT_WDF_DRIVER_DEVICE_ADD ForTunEvtDeviceAdd;
EVT_WDF_OBJECT_CONTEXT_CLEANUP ForTunEvtDriverContextCleanup;

EXTERN_C_END
