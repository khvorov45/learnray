use crate::windows_bindings as win;

pub fn output_debug_string(buf: &[u8]) {
	unsafe { win::OutputDebugStringA(&buf[0]) };
}
