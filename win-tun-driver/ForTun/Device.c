/*++

Module Name:

	device.c - Device handling events for example driver.

Abstract:

   This file contains the device entry points and callbacks.

Environment:

	Kernel-mode Driver Framework

--*/

#include "Device.h"
#include "Adapter.h"
#include "Queue.h"
#include "Public.h"
#include "device.tmh"

#ifdef ALLOC_PRAGMA
#pragma alloc_text (PAGE, ForTunCreateDevice)
#endif

NTSTATUS
ForTunCreateDevice(
	_Inout_ PWDFDEVICE_INIT DeviceInit
)
/*++

Routine Description:

	Worker routine called to create a device and its software resources.

Arguments:

	DeviceInit - Pointer to an opaque init structure. Memory for this
					structure will be freed by the framework when the WdfDeviceCreate
					succeeds. So don't access the structure after that point.

Return Value:

	NTSTATUS

--*/
{
	WDF_OBJECT_ATTRIBUTES deviceAttributes;
	PDEVICE_CONTEXT deviceContext;
	WDFDEVICE device;
	NTSTATUS status;


	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "%!FUNC! Entry");

	PAGED_CODE();

	WdfDeviceInitSetExclusive(DeviceInit, TRUE);


	WDF_FILEOBJECT_CONFIG fileConfig;
	WDF_FILEOBJECT_CONFIG_INIT(&fileConfig, WDF_NO_EVENT_CALLBACK, WDF_NO_EVENT_CALLBACK, ForTunFileCleanUp);
	WdfDeviceInitSetFileObjectConfig(DeviceInit, &fileConfig, WDF_NO_OBJECT_ATTRIBUTES);


	status = NetDeviceInitConfig(DeviceInit);
	if (!NT_SUCCESS(status)) {
		goto done;
	}

	WDF_OBJECT_ATTRIBUTES_INIT_CONTEXT_TYPE(&deviceAttributes, DEVICE_CONTEXT);


	deviceAttributes.EvtCleanupCallback = ForTunDeviceCleanUp;

	status = WdfDeviceCreate(&DeviceInit, &deviceAttributes, &device);

	if (!NT_SUCCESS(status)) {
		TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "WdfDeviceCreate failure: %x", status);
		goto done;
	}


	deviceContext = DeviceGetContext(device);	
	DECLARE_CONST_UNICODE_STRING(symLink, L"\\DosDevices\\ForTun");

	//
	// Create a device interface so that applications can find and talk
	// to us.
	//
	// If the client driver calls WdfDeviceCreateDeviceInterface with the ReferenceString parameter equal to NULL, 
	// NDIS intercepts I/O requests sent to the device interface. To avoid this behavior, specify any reference string.
	
	status = WdfDeviceCreateSymbolicLink(device, &symLink);
	if (!NT_SUCCESS(status)) {
		TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "create symLink failure: %x", status);
	}

	UNICODE_STRING referenceString;
	RtlInitUnicodeString(&referenceString, L"ForTun");


	status = WdfDeviceCreateDeviceInterface(
		device,
		&GUID_DEVINTERFACE_ForTun,
		&referenceString
	);

	if (!NT_SUCCESS(status)) {
		//This would fail if multiple device create..
		TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "create device interface failure: %x", status);
		goto done;
	}
	// Initialize the I/O Package and any Queues

	status = ForTunQueueInitialize(device);

	if (!NT_SUCCESS(status)) {
		TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "WdfDeviceCreate failure: %x", status);
		goto done;
	}

	status = ForTunAdapterCreate(deviceContext, device);


done:
	if (!NT_SUCCESS(status)) {
		TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "create device failure: %x", status);
	}
	return status;
}



VOID ForTunDeviceCleanUp(WDFOBJECT Obj) {

	UNREFERENCED_PARAMETER(Obj);
	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "%!FUNC! Entry");

	PDEVICE_CONTEXT pDeviceContext = DeviceGetContext(Obj);
	PoolQueueFree(pDeviceContext->PoolQueue);
	//pDeviceContext->Adapter = WDF_NO_HANDLE;

}

VOID ForTunFileCleanUp(
	_In_
	WDFFILEOBJECT FileObject
) {
	TraceEvents(TRACE_LEVEL_INFORMATION, TRACE_DEVICE, "%!FUNC! Entry");
	PDEVICE_CONTEXT context = DeviceGetContext(WdfFileObjectGetDevice(FileObject));

	if (context->Adapter != NULL) {
		ForTunAdapterSetLinkState(context->Adapter, MediaConnectStateDisconnected);
	}

}