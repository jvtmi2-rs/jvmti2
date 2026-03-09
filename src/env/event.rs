//! Event management methods on Env.

use super::Env;
use crate::{
    enums::{Event, EventMode},
    event::EventCallbacksBuilder,
    objects::JThread,
    sys,
};

impl<'local> Env<'local> {
    /// Enables or disables notification for an event type.
    ///
    /// If `thread` is `None`, the notification mode applies globally.
    ///
    /// # Phase
    /// May be called during the **OnLoad**, **live**, or for some events
    /// the **start** phase.
    pub fn set_event_notification_mode(
        &self,
        mode: EventMode,
        event_type: Event,
        thread: Option<&JThread<'_>>,
    ) -> crate::Result<()> {
        let thread_raw = thread.map_or(core::ptr::null_mut(), |t| t.as_raw());
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                SetEventNotificationMode,
                sys::jvmtiEventMode::from(mode),
                sys::jvmtiEvent::from(event_type),
                thread_raw
            )
        };
        Ok(())
    }

    /// Sets the raw event callbacks struct.
    ///
    /// Prefer using [`EventCallbacksBuilder`] for a type-safe interface.
    ///
    /// # Phase
    /// May be called during the **OnLoad**, **live**, or **start** phase.
    pub fn set_event_callbacks_raw(
        &self,
        callbacks: &sys::jvmtiEventCallbacks,
    ) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                SetEventCallbacks,
                callbacks as *const sys::jvmtiEventCallbacks,
                core::mem::size_of::<sys::jvmtiEventCallbacks>() as jni_sys::jint
            )
        };
        Ok(())
    }

    /// Applies the callbacks from an [`EventCallbacksBuilder`].
    pub fn set_event_callbacks(&self, builder: &EventCallbacksBuilder) -> crate::Result<()> {
        self.set_event_callbacks_raw(builder.as_raw())
    }

    /// Generates events for a given event type.
    ///
    /// This can be used to generate `CompiledMethodLoad` and
    /// `DynamicCodeGenerated` events.
    pub fn generate_events(&self, event_type: Event) -> crate::Result<()> {
        unsafe {
            jvmti_call_check!(
                self,
                v1,
                GenerateEvents,
                sys::jvmtiEvent::from(event_type)
            )
        };
        Ok(())
    }

    /// Installs an [`EventHandler`](crate::event::EventHandler) on this
    /// environment.
    ///
    /// Sets up C callback trampolines and stores the handler in
    /// environment-local storage. You must still enable each desired event
    /// with [`set_event_notification_mode`](Self::set_event_notification_mode).
    ///
    /// Returns an [`InstalledHandler`](crate::event::InstalledHandler) guard.
    /// Dropping it clears the callbacks and frees the handler.
    pub fn install_event_handler<H: crate::event::EventHandler>(
        &self,
        handler: H,
    ) -> crate::Result<crate::event::InstalledHandler> {
        crate::event::install_handler(self, handler)
    }
}
