/*++

Module Name:

    public.h

Abstract:

    This module contains the common declarations shared by driver
    and user applications.

Environment:

    user and kernel

--*/

//
// Define an Interface Guid so that apps can find the device and talk to it.
//
#pragma once

#include "initguid.h"


#define FOR_TUN_IOCTL_OPEN_ADAPTER CTL_CODE(FILE_DEVICE_UNKNOWN, 0x0801, METHOD_BUFFERED, FILE_ANY_ACCESS)



DEFINE_GUID (GUID_DEVINTERFACE_ForTun,
    0xf579d929,0x6c40,0x4e5a,0x85,0x32,0x18,0x01,0x99,0xa4,0xe3,0x21);
// {f579d929-6c40-4e5a-8532-180199a4e321}
