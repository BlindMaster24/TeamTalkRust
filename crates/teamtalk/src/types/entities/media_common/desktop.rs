//! Desktop sharing input packet types.

use teamtalk_sys as ffi;

/// Desktop input packet.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct DesktopInput {
    /// Mouse x-position in pixels.
    pub mouse_pos_x: u16,
    /// Mouse y-position in pixels.
    pub mouse_pos_y: u16,
    /// Virtual key code.
    pub key_code: u32,
    /// Key state flags.
    pub key_state: ffi::DesktopKeyStates,
}

impl From<ffi::DesktopInput> for DesktopInput {
    fn from(input: ffi::DesktopInput) -> Self {
        Self {
            mouse_pos_x: input.uMousePosX,
            mouse_pos_y: input.uMousePosY,
            key_code: input.uKeyCode,
            key_state: input.uKeyState,
        }
    }
}

impl DesktopInput {
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::DesktopInput {
        ffi::DesktopInput {
            uMousePosX: self.mouse_pos_x,
            uMousePosY: self.mouse_pos_y,
            uKeyCode: self.key_code,
            uKeyState: self.key_state,
        }
    }
}
