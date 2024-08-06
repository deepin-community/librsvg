#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn NtQueryInformationProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut ::core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn NtQueryInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut ::core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn NtSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const ::core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn NtWaitForSingleObject(handle : super::super::super::Win32::Foundation:: HANDLE, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("ntdll.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn ZwSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const ::core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
pub const MaxProcessInfoClass: PROCESSINFOCLASS = 83i32;
pub const MaxThreadInfoClass: THREADINFOCLASS = 56i32;
pub const ProcessAccessToken: PROCESSINFOCLASS = 9i32;
pub const ProcessAffinityMask: PROCESSINFOCLASS = 21i32;
pub const ProcessAffinityUpdateMode: PROCESSINFOCLASS = 45i32;
pub const ProcessBasePriority: PROCESSINFOCLASS = 5i32;
pub const ProcessBasicInformation: PROCESSINFOCLASS = 0i32;
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = 29i32;
pub const ProcessCheckStackExtentsMode: PROCESSINFOCLASS = 59i32;
pub const ProcessCommandLineInformation: PROCESSINFOCLASS = 60i32;
pub const ProcessCommitReleaseInformation: PROCESSINFOCLASS = 65i32;
pub const ProcessCookie: PROCESSINFOCLASS = 36i32;
pub const ProcessCycleTime: PROCESSINFOCLASS = 38i32;
pub const ProcessDebugFlags: PROCESSINFOCLASS = 31i32;
pub const ProcessDebugObjectHandle: PROCESSINFOCLASS = 30i32;
pub const ProcessDebugPort: PROCESSINFOCLASS = 7i32;
pub const ProcessDefaultHardErrorMode: PROCESSINFOCLASS = 12i32;
pub const ProcessDeviceMap: PROCESSINFOCLASS = 23i32;
pub const ProcessDynamicFunctionTableInformation: PROCESSINFOCLASS = 53i32;
pub const ProcessEnableAlignmentFaultFixup: PROCESSINFOCLASS = 17i32;
pub const ProcessEnergyTrackingState: PROCESSINFOCLASS = 82i32;
pub const ProcessExceptionPort: PROCESSINFOCLASS = 8i32;
pub const ProcessExecuteFlags: PROCESSINFOCLASS = 34i32;
pub const ProcessFaultInformation: PROCESSINFOCLASS = 63i32;
pub const ProcessForegroundInformation: PROCESSINFOCLASS = 25i32;
pub const ProcessGroupInformation: PROCESSINFOCLASS = 47i32;
pub const ProcessHandleCheckingMode: PROCESSINFOCLASS = 54i32;
pub const ProcessHandleCount: PROCESSINFOCLASS = 20i32;
pub const ProcessHandleInformation: PROCESSINFOCLASS = 51i32;
pub const ProcessHandleTable: PROCESSINFOCLASS = 58i32;
pub const ProcessHandleTracing: PROCESSINFOCLASS = 32i32;
pub const ProcessImageFileMapping: PROCESSINFOCLASS = 44i32;
pub const ProcessImageFileName: PROCESSINFOCLASS = 27i32;
pub const ProcessImageFileNameWin32: PROCESSINFOCLASS = 43i32;
pub const ProcessImageInformation: PROCESSINFOCLASS = 37i32;
pub const ProcessInPrivate: PROCESSINFOCLASS = 70i32;
pub const ProcessInstrumentationCallback: PROCESSINFOCLASS = 40i32;
pub const ProcessIoCounters: PROCESSINFOCLASS = 2i32;
pub const ProcessIoPortHandlers: PROCESSINFOCLASS = 13i32;
pub const ProcessIoPriority: PROCESSINFOCLASS = 33i32;
pub const ProcessKeepAliveCount: PROCESSINFOCLASS = 55i32;
pub const ProcessLUIDDeviceMapsEnabled: PROCESSINFOCLASS = 28i32;
pub const ProcessLdtInformation: PROCESSINFOCLASS = 10i32;
pub const ProcessLdtSize: PROCESSINFOCLASS = 11i32;
pub const ProcessMemoryAllocationMode: PROCESSINFOCLASS = 46i32;
pub const ProcessMemoryExhaustion: PROCESSINFOCLASS = 62i32;
pub const ProcessMitigationPolicy: PROCESSINFOCLASS = 52i32;
pub const ProcessOwnerInformation: PROCESSINFOCLASS = 49i32;
pub const ProcessPagePriority: PROCESSINFOCLASS = 39i32;
pub const ProcessPooledUsageAndLimits: PROCESSINFOCLASS = 14i32;
pub const ProcessPriorityBoost: PROCESSINFOCLASS = 22i32;
pub const ProcessPriorityClass: PROCESSINFOCLASS = 18i32;
pub const ProcessProtectionInformation: PROCESSINFOCLASS = 61i32;
pub const ProcessQuotaLimits: PROCESSINFOCLASS = 1i32;
pub const ProcessRaisePriority: PROCESSINFOCLASS = 6i32;
pub const ProcessRaiseUMExceptionOnInvalidHandleClose: PROCESSINFOCLASS = 71i32;
pub const ProcessReserved1Information: PROCESSINFOCLASS = 66i32;
pub const ProcessReserved2Information: PROCESSINFOCLASS = 67i32;
pub const ProcessRevokeFileHandles: PROCESSINFOCLASS = 56i32;
pub const ProcessSessionInformation: PROCESSINFOCLASS = 24i32;
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = 75i32;
pub const ProcessSubsystemProcess: PROCESSINFOCLASS = 68i32;
pub const ProcessTelemetryIdInformation: PROCESSINFOCLASS = 64i32;
pub const ProcessThreadStackAllocation: PROCESSINFOCLASS = 41i32;
pub const ProcessTimes: PROCESSINFOCLASS = 4i32;
pub const ProcessTlsInformation: PROCESSINFOCLASS = 35i32;
pub const ProcessTokenVirtualizationEnabled: PROCESSINFOCLASS = 48i32;
pub const ProcessUserModeIOPL: PROCESSINFOCLASS = 16i32;
pub const ProcessVmCounters: PROCESSINFOCLASS = 3i32;
pub const ProcessWin32kSyscallFilterInformation: PROCESSINFOCLASS = 79i32;
pub const ProcessWindowInformation: PROCESSINFOCLASS = 50i32;
pub const ProcessWorkingSetControl: PROCESSINFOCLASS = 57i32;
pub const ProcessWorkingSetWatch: PROCESSINFOCLASS = 15i32;
pub const ProcessWorkingSetWatchEx: PROCESSINFOCLASS = 42i32;
pub const ProcessWow64Information: PROCESSINFOCLASS = 26i32;
pub const ProcessWx86Information: PROCESSINFOCLASS = 19i32;
pub const ThreadActualBasePriority: THREADINFOCLASS = 25i32;
pub const ThreadActualGroupAffinity: THREADINFOCLASS = 41i32;
pub const ThreadAffinityMask: THREADINFOCLASS = 4i32;
pub const ThreadAmILastThread: THREADINFOCLASS = 12i32;
pub const ThreadBasePriority: THREADINFOCLASS = 3i32;
pub const ThreadBasicInformation: THREADINFOCLASS = 0i32;
pub const ThreadBreakOnTermination: THREADINFOCLASS = 18i32;
pub const ThreadCSwitchMon: THREADINFOCLASS = 27i32;
pub const ThreadCSwitchPmu: THREADINFOCLASS = 28i32;
pub const ThreadCounterProfiling: THREADINFOCLASS = 32i32;
pub const ThreadCpuAccountingInformation: THREADINFOCLASS = 34i32;
pub const ThreadCycleTime: THREADINFOCLASS = 23i32;
pub const ThreadDescriptorTableEntry: THREADINFOCLASS = 6i32;
pub const ThreadDynamicCodePolicyInfo: THREADINFOCLASS = 42i32;
pub const ThreadEnableAlignmentFaultFixup: THREADINFOCLASS = 7i32;
pub const ThreadEventPair_Reusable: THREADINFOCLASS = 8i32;
pub const ThreadGroupInformation: THREADINFOCLASS = 30i32;
pub const ThreadHideFromDebugger: THREADINFOCLASS = 17i32;
pub const ThreadIdealProcessor: THREADINFOCLASS = 13i32;
pub const ThreadIdealProcessorEx: THREADINFOCLASS = 33i32;
pub const ThreadImpersonationToken: THREADINFOCLASS = 5i32;
pub const ThreadIoPriority: THREADINFOCLASS = 22i32;
pub const ThreadIsIoPending: THREADINFOCLASS = 16i32;
pub const ThreadIsTerminated: THREADINFOCLASS = 20i32;
pub const ThreadLastSystemCall: THREADINFOCLASS = 21i32;
pub const ThreadPagePriority: THREADINFOCLASS = 24i32;
pub const ThreadPerformanceCount: THREADINFOCLASS = 11i32;
pub const ThreadPriority: THREADINFOCLASS = 2i32;
pub const ThreadPriorityBoost: THREADINFOCLASS = 14i32;
pub const ThreadQuerySetWin32StartAddress: THREADINFOCLASS = 9i32;
pub const ThreadSetTlsArrayAddress: THREADINFOCLASS = 15i32;
pub const ThreadSubsystemInformation: THREADINFOCLASS = 45i32;
pub const ThreadSuspendCount: THREADINFOCLASS = 35i32;
pub const ThreadSwitchLegacyState: THREADINFOCLASS = 19i32;
pub const ThreadTebInformation: THREADINFOCLASS = 26i32;
pub const ThreadTimes: THREADINFOCLASS = 1i32;
pub const ThreadUmsInformation: THREADINFOCLASS = 31i32;
pub const ThreadWow64Context: THREADINFOCLASS = 29i32;
pub const ThreadZeroTlsCell: THREADINFOCLASS = 10i32;
pub type PROCESSINFOCLASS = i32;
pub type THREADINFOCLASS = i32;
