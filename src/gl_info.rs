//! Cached OpenGL information.

use crate::gl_limits::GLLimits;
use gleam::gl::GlType;

bitflags! {
    // https://www.khronos.org/registry/webgl/specs/latest/1.0/#WEBGLCONTEXTATTRIBUTES
    pub struct ContextAttributes: u8 {
        const ALPHA   = 0x01;
        const DEPTH   = 0x02;
        const STENCIL = 0x04;
    }
}

bitflags! {
    pub struct FeatureFlags: u8 {
        const SUPPORTS_DEPTH24_STENCIL8 = 0x01;
    }
}

#[derive(Clone, Copy)]
pub struct GLInfo {
    pub flavor: GLFlavor,
    pub limits: GLLimits,
    pub attributes: ContextAttributes,
    pub features: FeatureFlags,
}

/// The OpenGL API and its associated version.
#[derive(Clone, Copy, Debug)]
pub struct GLFlavor {
    pub api_type: GlType,
    pub api_version: GLVersion,
}

/// Describes the OpenGL version that is requested when a context is created.
#[derive(Debug, Clone, Copy)]
pub enum GLVersion {
    /// Request a specific major version
    /// The minor version is automatically selected.
    Major(u8),

    /// Request a specific major and minor version version.
    MajorMinor(u8, u8),
}

impl GLVersion {
    // Helper method to get the major version
    pub fn major_version(&self) -> u8 {
        match *self {
            GLVersion::Major(major) => major,
            GLVersion::MajorMinor(major, _) => major,
        }
    }
}
