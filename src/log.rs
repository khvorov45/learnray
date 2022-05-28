pub struct DebugOutput {
    pub buf: [u8; 1024],
    used: usize,
}

impl Default for DebugOutput {
    fn default() -> Self {
        Self {
            buf: [0; 1024],
            used: 0,
        }
    }
}

impl core::fmt::Write for DebugOutput {
    fn write_str(&mut self, msg: &str) -> Result<(), core::fmt::Error> {
        let mut result = Ok(());
        for byte in msg.bytes() {
            // NOTE(khvorov) Null terminator
            if self.used < self.buf.len() - 1 {
                self.buf[self.used] = byte;
                self.used += 1;
            } else {
                result = Err(core::fmt::Error::default());
                break;
            }
        }
        result
    }
}

macro_rules! log_debug {
    // log_debug!("a {} event", "log")
    ($($arg:tt)+) => ({
        use core::fmt::Write;
        let mut output = crate::log::DebugOutput::default();
        if write!(&mut output, $($arg)+).is_ok() {
            #[allow(unused_unsafe)]
            #[cfg(target_os = "windows")]
            unsafe { crate::platform_log::output_debug_string(&output.buf) };
        }
    })
}
