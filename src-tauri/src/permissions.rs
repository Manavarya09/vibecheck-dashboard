#[cfg(target_os = "macos")]
pub fn check_screen_recording() -> bool {
    // Check if window titles are accessible by trying to read one
    // If Screen Recording permission isn't granted, titles come back empty
    match active_win_pos_rs::get_active_window() {
        Ok(window) => !window.title.is_empty(),
        Err(_) => false,
    }
}

#[cfg(not(target_os = "macos"))]
pub fn check_screen_recording() -> bool {
    true
}
