//! Bitflag types wrapping JVMTI bitmask constants.

use bitflags::bitflags;

bitflags! {
    /// Thread state flags.
    ///
    /// These can be combined to describe the full state of a thread.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ThreadState: u32 {
        /// Thread is alive.
        const ALIVE = jvmti2_sys::JVMTI_THREAD_STATE_ALIVE;
        /// Thread has terminated.
        const TERMINATED = jvmti2_sys::JVMTI_THREAD_STATE_TERMINATED;
        /// Thread is runnable.
        const RUNNABLE = jvmti2_sys::JVMTI_THREAD_STATE_RUNNABLE;
        /// Thread is blocked on monitor enter.
        const BLOCKED_ON_MONITOR_ENTER = jvmti2_sys::JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER;
        /// Thread is waiting.
        const WAITING = jvmti2_sys::JVMTI_THREAD_STATE_WAITING;
        /// Thread is waiting indefinitely.
        const WAITING_INDEFINITELY = jvmti2_sys::JVMTI_THREAD_STATE_WAITING_INDEFINITELY;
        /// Thread is waiting with timeout.
        const WAITING_WITH_TIMEOUT = jvmti2_sys::JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;
        /// Thread is sleeping.
        const SLEEPING = jvmti2_sys::JVMTI_THREAD_STATE_SLEEPING;
        /// Thread is in `Object.wait()`.
        const IN_OBJECT_WAIT = jvmti2_sys::JVMTI_THREAD_STATE_IN_OBJECT_WAIT;
        /// Thread is parked.
        const PARKED = jvmti2_sys::JVMTI_THREAD_STATE_PARKED;
        /// Thread is suspended.
        const SUSPENDED = jvmti2_sys::JVMTI_THREAD_STATE_SUSPENDED;
        /// Thread was interrupted.
        const INTERRUPTED = jvmti2_sys::JVMTI_THREAD_STATE_INTERRUPTED;
        /// Thread is in native code.
        const IN_NATIVE = jvmti2_sys::JVMTI_THREAD_STATE_IN_NATIVE;
        /// Vendor-specific flag 1.
        const VENDOR_1 = jvmti2_sys::JVMTI_THREAD_STATE_VENDOR_1;
        /// Vendor-specific flag 2.
        const VENDOR_2 = jvmti2_sys::JVMTI_THREAD_STATE_VENDOR_2;
        /// Vendor-specific flag 3.
        const VENDOR_3 = jvmti2_sys::JVMTI_THREAD_STATE_VENDOR_3;
    }
}

bitflags! {
    /// Class status flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ClassStatus: u32 {
        /// Class has been verified.
        const VERIFIED = jvmti2_sys::JVMTI_CLASS_STATUS_VERIFIED;
        /// Class has been prepared.
        const PREPARED = jvmti2_sys::JVMTI_CLASS_STATUS_PREPARED;
        /// Class has been initialized.
        const INITIALIZED = jvmti2_sys::JVMTI_CLASS_STATUS_INITIALIZED;
        /// Class has an initialization error.
        const ERROR = jvmti2_sys::JVMTI_CLASS_STATUS_ERROR;
        /// Class is an array class.
        const ARRAY = jvmti2_sys::JVMTI_CLASS_STATUS_ARRAY;
        /// Class is a primitive class.
        const PRIMITIVE = jvmti2_sys::JVMTI_CLASS_STATUS_PRIMITIVE;
    }
}

bitflags! {
    /// Heap filter flags for `FollowReferences` and `IterateThroughHeap`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HeapFilter: u32 {
        /// Filter out tagged objects.
        const TAGGED = jvmti2_sys::JVMTI_HEAP_FILTER_TAGGED;
        /// Filter out untagged objects.
        const UNTAGGED = jvmti2_sys::JVMTI_HEAP_FILTER_UNTAGGED;
        /// Filter out objects whose class is tagged.
        const CLASS_TAGGED = jvmti2_sys::JVMTI_HEAP_FILTER_CLASS_TAGGED;
        /// Filter out objects whose class is untagged.
        const CLASS_UNTAGGED = jvmti2_sys::JVMTI_HEAP_FILTER_CLASS_UNTAGGED;
    }
}
