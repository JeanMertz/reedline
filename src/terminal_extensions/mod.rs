pub(crate) mod bracketed_paste;
pub(crate) mod kitty;
pub mod semantic_prompt;

/// Return if the terminal supports the kitty keyboard enhancement protocol
///
/// Read more: <https://sw.kovidgoyal.net/kitty/keyboard-protocol/>
///
/// SIDE EFFECT: Touches the terminal file descriptors
pub fn kitty_protocol_available() -> bool {
    crossterm::terminal::supports_keyboard_enhancement().unwrap_or_default()
}

/// Write a terminal-mode teardown command to the controlling terminal.
///
/// Used from `Drop` impls, where no caller-supplied writer is available.
/// Teardown must reach the terminal even when stdout is redirected: writing to
/// a redirected stdout would corrupt the caller's output *and* leave the
/// terminal mode active.
/// Prefer `/dev/tty`, falling back to stdout when no controlling terminal is
/// available (e.g. Windows or detached processes).
pub(crate) fn emergency_teardown(command: impl crossterm::Command) {
    #[cfg(unix)]
    if let Ok(mut tty) = std::fs::OpenOptions::new().write(true).open("/dev/tty") {
        let _ = crossterm::execute!(tty, command);
        return;
    }
    let _ = crossterm::execute!(std::io::stdout(), command);
}
