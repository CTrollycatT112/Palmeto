// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This kernel uses NTSTATUS style
//          UNIX like systems commonly use 'ERRNO'
//          There will be an ERRNO layer for system calls
//          But my KERNEL uses an NTSTATUS based system
//          This allows for more verbose error handling

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Status(pub u32);

impl Status {
    pub fn severity(&self) -> u32 {
        self.0 >> 30
    }

    pub fn is_success(&self) -> bool {
        self.severity() == 0
    }

    pub fn is_info(&self) -> bool {
        self.severity() == 1
    }

    pub fn is_warning(&self) -> bool {
        self.severity() == 2
    }

    pub fn is_error(&self) -> bool {
        self.severity() == 3
    }
}

pub type KResult<T> = Result<T, Status>;

pub trait StatusResultExt<T> {
    fn into_status(self) -> Status;
}

impl<T> StatusResultExt<T> for KResult<T> {
    fn into_status(self) -> Status {
        match self {
            Ok(_) => Status::SUCCESS,
            Err(s) => s,
        }
    }
}

impl Status {
    // SUCCESS
    pub const SUCCESS: Self = Status(0x00000000);
    pub const ALREADY_COMPLETE: Self = Status(0x000000FF);
    pub const NOTIFY_CLEANUP: Self = Status(0x0000010B);
    pub const NOTIFY_ENUM_DIR: Self = Status(0x0000010C);
    pub const PENDING: Self = Status(0x00000103);
    pub const REPARSE: Self = Status(0x00000104);
    pub const MORE_ENTRIES: Self = Status(0x00000105);
    pub const NOT_ALL_ASSIGNED: Self = Status(0x00000106);
    pub const SOME_NOT_MAPPED: Self = Status(0x00000107);
    pub const TIMEOUT: Self = Status(0x00000102);
    pub const ALERTED: Self = Status(0x00000101);
    pub const USER_APC: Self = Status(0x000000C0);
    pub const WAIT_0: Self = Status(0x00000000);
    pub const WAIT_1: Self = Status(0x00000001);
    pub const WAIT_2: Self = Status(0x00000002);
    pub const WAIT_3: Self = Status(0x00000003);
    pub const ABANDONED: Self = Status(0x00000080);

    // INFO
    pub const OBJECT_NAME_EXISTS: Self = Status(0x40000000);
    pub const THREAD_WAS_SUSPENDED: Self = Status(0x40000001);
    pub const WORKING_SET_LIMIT_RANGE: Self = Status(0x40000002);
    pub const IMAGE_NOT_AT_BASE: Self = Status(0x40000003);
    pub const RXACT_STATE_CREATED: Self = Status(0x40000004);
    pub const SEGMENT_NOTIFICATION: Self = Status(0x40000005);
    pub const LOCAL_USER_SESSION_KEY: Self = Status(0x40000006);
    pub const BAD_CURRENT_DIRECTORY: Self = Status(0x40000007);
    pub const SERIAL_MORE_WRITES: Self = Status(0x40000008);
    pub const REGISTRY_RECOVERED: Self = Status(0x40000009);
    pub const FT_READ_RECOVERY_FROM_BACKUP: Self = Status(0x4000000A);
    pub const FT_WRITE_RECOVERY: Self = Status(0x4000000B);
    pub const SERIAL_COUNTER_TIMEOUT: Self = Status(0x4000000C);
    pub const NULL_LM_PASSWORD: Self = Status(0x4000000D);
    pub const IMAGE_MACHINE_TYPE_MISMATCH: Self = Status(0x4000000E);
    pub const RECEIVE_PARTIAL: Self = Status(0x4000000F);
    pub const RECEIVE_EXPEDITED: Self = Status(0x40000010);
    pub const RECEIVE_PARTIAL_EXPEDITED: Self = Status(0x40000011);
    pub const EVENT_DONE: Self = Status(0x40000012);
    pub const EVENT_PENDING: Self = Status(0x40000013);
    pub const CHECKING_FILE_SYSTEM: Self = Status(0x40000014);
    pub const FATAL_APP_EXIT: Self = Status(0x40000015);
    pub const PREDEFINED_HANDLE: Self = Status(0x40000016);
    pub const WAS_UNLOCKED: Self = Status(0x40000017);
    pub const SERVICE_NOTIFICATION: Self = Status(0x40000018);
    pub const WAS_LOCKED: Self = Status(0x40000019);
    pub const LOG_HARD_ERROR: Self = Status(0x4000001A);
    pub const ALREADY_WIN32: Self = Status(0x4000001B);
    pub const WX86_UNSIMULATE: Self = Status(0x4000001C);
    pub const WX86_CONTINUE: Self = Status(0x4000001D);
    pub const WX86_SINGLE_STEP: Self = Status(0x4000001E);
    pub const WX86_BREAKPOINT: Self = Status(0x4000001F);
    pub const WX86_EXCEPTION_CONTINUE: Self = Status(0x40000020);
    pub const WX86_EXCEPTION_LASTCHANCE: Self = Status(0x40000021);
    pub const WX86_EXCEPTION_CHAIN: Self = Status(0x40000022);
    pub const IMAGE_MACHINE_TYPE_MISMATCH_EXE: Self = Status(0x40000023);
    pub const NO_YIELD_PERFORMED: Self = Status(0x40000024);
    pub const TIMER_RESUME_IGNORED: Self = Status(0x40000025);
    pub const ARBITRATION_UNHANDLED: Self = Status(0x40000026);
    pub const CARDBUS_NOT_SUPPORTED: Self = Status(0x40000027);
    pub const WX86_CREATEWX86TIB: Self = Status(0x40000028);
    pub const MP_PROCESSOR_MISMATCH: Self = Status(0x40000029);
    pub const HIBERNATED: Self = Status(0x4000002A);
    pub const RESUME_HIBERNATION: Self = Status(0x4000002B);
    pub const FIRMWARE_UPDATED: Self = Status(0x4000002C);

    // WARNING
    pub const GUARD_PAGE_VIOLATION: Self = Status(0x80000001);
    pub const DATATYPE_MISALIGNMENT: Self = Status(0x80000002);
    pub const BREAKPOINT: Self = Status(0x80000003);
    pub const SINGLE_STEP: Self = Status(0x80000004);
    pub const BUFFER_OVERFLOW: Self = Status(0x80000005);
    pub const NO_MORE_FILES: Self = Status(0x80000006);
    pub const HANDLES_CLOSED: Self = Status(0x8000000A);
    pub const PARTIAL_COPY: Self = Status(0x8000000D);
    pub const DEVICE_PAPER_EMPTY: Self = Status(0x8000000E);
    pub const DEVICE_POWERED_OFF: Self = Status(0x8000000F);
    pub const DEVICE_OFF_LINE: Self = Status(0x80000010);
    pub const DEVICE_BUSY: Self = Status(0x80000011);
    pub const NO_MORE_EAS: Self = Status(0x80000012);
    pub const INVALID_EA_NAME: Self = Status(0x80000013);
    pub const EA_LIST_INCONSISTENT: Self = Status(0x80000014);
    pub const INVALID_EA_FLAG: Self = Status(0x80000015);
    pub const VERIFY_REQUIRED: Self = Status(0x80000016);
    pub const EXTRANEOUS_INFORMATION: Self = Status(0x80000017);
    pub const RXACT_COMMIT_NECESSARY: Self = Status(0x80000018);
    pub const NO_MORE_ENTRIES: Self = Status(0x8000001A);
    pub const FILEMARK_DETECTED: Self = Status(0x8000001B);
    pub const MEDIA_CHANGED: Self = Status(0x8000001C);
    pub const BUS_RESET: Self = Status(0x8000001D);
    pub const END_OF_MEDIA: Self = Status(0x8000001E);
    pub const BEGINNING_OF_MEDIA: Self = Status(0x8000001F);
    pub const MEDIA_CHECK: Self = Status(0x80000020);
    pub const SETMARK_DETECTED: Self = Status(0x80000021);
    pub const NO_DATA_DETECTED: Self = Status(0x80000022);
    pub const REDIRECTOR_HAS_OPEN_HANDLES: Self = Status(0x80000023);
    pub const SERVER_HAS_OPEN_HANDLES: Self = Status(0x80000024);
    pub const ALREADY_DISCONNECTED: Self = Status(0x80000025);
    pub const LONGJUMP: Self = Status(0x80000026);
    pub const CLEANER_CARTRIDGE_INSTALLED: Self = Status(0x80000027);
    pub const PLUGPLAY_QUERY_VETOED: Self = Status(0x80000028);
    pub const UNWIND_CONSOLIDATE: Self = Status(0x80000029);
    pub const REGISTRY_HIVE_RECOVERED: Self = Status(0x8000002A);
    pub const DLL_MIGHT_BE_INSECURE: Self = Status(0x8000002B);
    pub const DLL_MIGHT_BE_INCOMPATIBLE: Self = Status(0x8000002C);
    pub const STOPPED_ON_SYMLINK: Self = Status(0x8000002D);

    // ERROR
    pub const UNSUCCESSFUL: Self = Status(0xC0000001);
    pub const NOT_IMPLEMENTED: Self = Status(0xC0000002);
    pub const INVALID_INFO_CLASS: Self = Status(0xC0000003);
    pub const INFO_LENGTH_MISMATCH: Self = Status(0xC0000004);
    pub const ACCESS_VIOLATION: Self = Status(0xC0000005);
    pub const IN_PAGE_ERROR: Self = Status(0xC0000006);
    pub const PAGEFILE_QUOTA: Self = Status(0xC0000007);
    pub const INVALID_HANDLE: Self = Status(0xC0000008);
    pub const BAD_INITIAL_STACK: Self = Status(0xC0000009);
    pub const BAD_INITIAL_PC: Self = Status(0xC000000A);
    pub const INVALID_CID: Self = Status(0xC000000B);
    pub const TIMER_NOT_CANCELED: Self = Status(0xC000000C);
    pub const INVALID_PARAMETER: Self = Status(0xC000000D);
    pub const NO_SUCH_DEVICE: Self = Status(0xC000000E);
    pub const NO_SUCH_FILE: Self = Status(0xC000000F);
    pub const INVALID_DEVICE_REQUEST: Self = Status(0xC0000010);
    pub const END_OF_FILE: Self = Status(0xC0000011);
    pub const WRONG_VOLUME: Self = Status(0xC0000012);
    pub const NO_MEDIA_IN_DEVICE: Self = Status(0xC0000013);
    pub const UNRECOGNIZED_MEDIA: Self = Status(0xC0000014);
    pub const NONEXISTENT_SECTOR: Self = Status(0xC0000015);
    pub const MORE_PROCESSING_REQUIRED: Self = Status(0xC0000016);
    pub const NO_MEMORY: Self = Status(0xC0000017);
    pub const CONFLICTING_ADDRESSES: Self = Status(0xC0000018);
    pub const NOT_MAPPED_VIEW: Self = Status(0xC0000019);
    pub const UNABLE_TO_FREE_VM: Self = Status(0xC000001A);
    pub const UNABLE_TO_DELETE_SECTION: Self = Status(0xC000001B);
    pub const INVALID_SYSTEM_SERVICE: Self = Status(0xC000001C);
    pub const ILLEGAL_INSTRUCTION: Self = Status(0xC000001D);
    pub const INVALID_LOCK_SEQUENCE: Self = Status(0xC000001E);
    pub const INVALID_VIEW_SIZE: Self = Status(0xC000001F);
    pub const INVALID_FILE_FOR_SECTION: Self = Status(0xC0000020);
    pub const ALREADY_COMMITTED: Self = Status(0xC0000021);
    pub const ACCESS_DENIED: Self = Status(0xC0000022);
    pub const BUFFER_TOO_SMALL: Self = Status(0xC0000023);
    pub const OBJECT_TYPE_MISMATCH: Self = Status(0xC0000024);
    pub const NONCONTINUABLE_EXCEPTION: Self = Status(0xC0000025);
    pub const INVALID_DISPOSITION: Self = Status(0xC0000026);
    pub const UNWIND: Self = Status(0xC0000027);
    pub const BAD_STACK: Self = Status(0xC0000028);
    pub const INVALID_UNWIND_TARGET: Self = Status(0xC0000029);
    pub const NOT_LOCKED: Self = Status(0xC000002A);
    pub const PARITY_ERROR: Self = Status(0xC000002B);
    pub const UNABLE_TO_DECOMMIT_VM: Self = Status(0xC000002C);
    pub const NOT_COMMITTED: Self = Status(0xC000002D);
    pub const INVALID_PORT_ATTRIBUTES: Self = Status(0xC000002E);
    pub const PORT_MESSAGE_TOO_LONG: Self = Status(0xC000002F);
    pub const INVALID_PARAMETER_MIX: Self = Status(0xC0000030);
    pub const NOT_SUPPORTED: Self = Status(0xC00000BB);
    pub const DEVICE_NOT_READY: Self = Status(0xC00000A3);
    pub const NETWORK_UNREACHABLE: Self = Status(0xC000023C);
    pub const CONNECTION_REFUSED: Self = Status(0xC0000236);
    pub const CONNECTION_DISCONNECTED: Self = Status(0xC000020C);
    pub const IO_TIMEOUT: Self = Status(0xC00000B5);
    pub const DISK_FULL: Self = Status(0xC000007F);
    pub const FILE_IS_A_DIRECTORY: Self = Status(0xC00000BA);
    pub const NOT_A_DIRECTORY: Self = Status(0xC0000103);
    pub const DIRECTORY_NOT_EMPTY: Self = Status(0xC0000101);
    pub const OBJECT_NAME_NOT_FOUND: Self = Status(0xC0000034);
    pub const OBJECT_NAME_COLLISION: Self = Status(0xC0000035);
    pub const OBJECT_PATH_INVALID: Self = Status(0xC0000039);
    pub const OBJECT_PATH_NOT_FOUND: Self = Status(0xC000003A);
    pub const OBJECT_PATH_SYNTAX_BAD: Self = Status(0xC000003B);
    pub const SHARING_VIOLATION: Self = Status(0xC0000043);
    pub const QUOTA_EXCEEDED: Self = Status(0xC0000044);
    pub const INVALID_PAGE_PROTECTION: Self = Status(0xC0000045);
    pub const MUTANT_NOT_OWNED: Self = Status(0xC0000046);
    pub const SEMAPHORE_LIMIT_EXCEEDED: Self = Status(0xC0000047);
    pub const INSUFFICIENT_RESOURCES: Self = Status(0xC000009A);
    pub const STACK_OVERFLOW: Self = Status(0xC00000FD);
    pub const NOT_FOUND: Self = Status(0xC0000225);
    pub const PRIVILEGE_NOT_HELD: Self = Status(0xC0000061);
    pub const LOGON_FAILURE: Self = Status(0xC000006D);
    pub const FILE_CORRUPT_ERROR: Self = Status(0xC0000102);
    pub const DEVICE_DOES_NOT_EXIST: Self = Status(0xC00000C0);
    pub const TOO_MANY_OPENED_FILES: Self = Status(0xC000011F);
    pub const CANCELLED: Self = Status(0xC0000120);
    pub const DELETE_PENDING: Self = Status(0xC0000056);
    pub const NAME_TOO_LONG: Self = Status(0xC0000106);
    pub const INVALID_ADDRESS: Self = Status(0xC0000141);
    pub const NOT_SAME_DEVICE: Self = Status(0xC00000D4);
}