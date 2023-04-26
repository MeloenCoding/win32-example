

use window::{Window};
use windows::{Win32::{UI::{WindowsAndMessaging::{CS_OWNDC}, Input::KeyboardAndMouse::{VK_RETURN}}}, s};

mod window;

fn main() {
    // If you want to test the custom error, just uncomment this:
    // window::error::WindowError::new("Unable to load cursor.", None, loc!());

    // create a new window with name "Test". See window.rs for more info
    let mut window: Window = Window::new(s!("Test"), CS_OWNDC);

    // show the window you've created
    window.show_window(); 

    let mut input_str: String = "".to_string();

    loop {
        match window.handle_message() {
            Err((_msg, _result)) => {
                // getMessage = (0 = there is an exit without an error) | ( -1 = there is an exit with an error)
                break;
            },
            Ok(_msg) => {
                if window.keyboard.key_is_pressed_clear(VK_RETURN.0) {
                    println!("{:?}", input_str);
                    input_str = "".to_string();
                }
                if let Some(ch) = window.keyboard.read_char() {
                    input_str.push(ch);
                }
            },
        }
    }

    // print the exit codes
    window.get_exit_codes();
}
