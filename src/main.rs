use window::{Window};
use windows::{Win32::{UI::{WindowsAndMessaging::CS_OWNDC, Input::KeyboardAndMouse::{VK_RETURN}}}, s};

mod window;

fn main() {
    // Create a new window with name "Test". See window.rs for more info.
    let mut window: Window = Window::new(s!("Test"), CS_OWNDC, 800, 650);

    // Show the window you've created.
    window.show_window(); 

    let mut input_str: String = "".to_string();
    loop {
        match window.handle_message() {
            Err((_msg, result)) => {
                if result == -1 {
                    let (error_desc, error_code) = window.get_error_desc();
                    window::error::WindowError::new(&error_desc, Some(error_code as i32), loc!());
                }
                break;
            },
            Ok(_msg) => {
                if let Some(ch) = window.keyboard.read_char() {
                    input_str.push(ch);
                }
                if window.keyboard.key_is_pressed_pop(VK_RETURN.0) {
                    println!("{:?}", input_str);
                    input_str = "".to_string();
                }
            },
        }
    }

    // print the exit codes
    window.print_exit_codes();
}
