// surfman/surfman/src/platform/android/connection.rs
//
//! A no-op connection for Android.
//! 
//! FIXME(pcwalton): Should this instead wrap `EGLDisplay`? Is that thread-safe on Android?

use crate::Error;
use super::device::{Adapter, Device, NativeDevice};
use super::surface::NativeWidget;

#[cfg(feature = "sm-winit")]
use winit::Window;

/// A connection to the display server.
#[derive(Clone)]
pub struct Connection;

/// An empty placeholder for native connections.
#[derive(Clone)]
pub struct NativeConnection;

impl Connection {
    /// Connects to the default display.
    #[inline]
    pub fn new() -> Result<Connection, Error> {
        Ok(Connection)
    }

    /// An alias for `Connection::new()`, present for consistency with other backends.
    #[inline]
    pub unsafe fn from_native_connection(_: NativeConnection) -> Result<Connection, Error> {
        Connection::new()
    }

    /// Returns the underlying native connection.
    #[inline]
    pub fn native_connection(&self) -> NativeConnection {
        NativeConnection
    }

    /// Returns the "best" adapter on this system.
    /// 
    /// This is an alias for `Connection::create_hardware_adapter()`.
    #[inline]
    pub fn create_adapter(&self) -> Result<Adapter, Error> {
        self.create_hardware_adapter()
    }

    /// Returns the "best" adapter on this system, preferring high-performance hardware adapters.
    #[inline]
    pub fn create_hardware_adapter(&self) -> Result<Adapter, Error> {
        Ok(Adapter)
    }

    /// Returns the "best" adapter on this system, preferring low-power hardware adapters.
    #[inline]
    pub fn create_low_power_adapter(&self) -> Result<Adapter, Error> {
        Ok(Adapter)
    }

    /// Returns the "best" adapter on this system, preferring software adapters.
    #[inline]
    pub fn create_software_adapter(&self) -> Result<Adapter, Error> {
        Ok(Adapter)
    }

    /// Opens the hardware device corresponding to the given adapter.
    /// 
    /// Device handles are local to a single thread.
    #[inline]
    pub fn create_device(&self, _: &Adapter) -> Result<Device, Error> {
        Device::new()
    }

    /// Wraps an Android `EGLDisplay` in a device and returns it.
    ///
    /// The underlying `EGLDisplay` is not retained, as there is no way to do this in the EGL API.
    /// Therefore, it is the caller's responsibility to keep it alive as long as this `Device`
    /// remains alive.
    #[inline]
    pub unsafe fn create_device_from_native_device(&self, native_device: NativeDevice)
                                                   -> Result<Device, Error> {
        Ok(Device { egl_display: native_device.0, display_is_owned: false })
    }

    /// Opens the display connection corresponding to the given `winit` window.
    #[cfg(feature = "sm-winit")]
    #[inline]
    pub fn from_winit_window(_: &Window) -> Result<Connection, Error> {
        Err(Error::UnsupportedOnThisPlatform)
    }

    /// Creates a native widget type from the given `winit` window.
    /// 
    /// This type can be later used to create surfaces that render to the window.
    #[cfg(feature = "sm-winit")]
    #[inline]
    pub fn create_native_widget_from_winit_window(&self, _: &Window)
                                                  -> Result<NativeWidget, Error> {
        Err(Error::UnsupportedOnThisPlatform)
    }
}

impl NativeConnection {
    /// Creates a native connection.
    ///
    /// This is a no-op method present for consistency with other backends.
    #[inline]
    pub fn current() -> Result<NativeConnection, Error> {
        Ok(NativeConnection)
    }
}
