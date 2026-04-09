use crate::error::AppError;

pub struct DetectedWindow {
    pub app_name: String,
    pub window_title: String,
}

pub fn get_active_window_info() -> Result<DetectedWindow, AppError> {
    match active_win_pos_rs::get_active_window() {
        Ok(window) => Ok(DetectedWindow {
            app_name: window.app_name,
            window_title: window.title,
        }),
        Err(()) => Err(AppError::WindowDetection(
            "Failed to get active window".to_string(),
        )),
    }
}
