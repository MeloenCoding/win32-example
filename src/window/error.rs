use windows::{Win32::{UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK}}, core::PCSTR};

#[derive(Debug)]
pub struct WindowError {
    pub details: String,
    pub origin: CallLocation
} 

#[derive(Debug)]
pub struct CallLocation {
    pub file: String,
    pub line: u32,
    pub column: u32
}

#[macro_export] 
macro_rules! loc {
    () => {
        crate::window::error::CallLocation { file: file!().to_string(), line: line!(), column: column!() }
    }
}

impl WindowError {
    pub fn new(error_details: &str, error_code: Option<i32>, origin: CallLocation) -> ! {
        let base_details: String = format!("Error in {}:{}\n{}{}", origin.file, origin.line, error_details, '\0');
        let formatted_details: PCSTR = PCSTR::from_raw(base_details.as_ptr());

        crate::window::create_message_box(formatted_details, MB_ICONERROR | MB_OK, 0);
        
        std::process::exit(error_code.unwrap_or(1));
    }
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for WindowError {
    fn description(&self) -> &str {
        &self.details
    }
}
