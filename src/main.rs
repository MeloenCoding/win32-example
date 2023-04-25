use std::{borrow::BorrowMut, ops::Deref};

use window::{Window, keyboard::KeyState};
use windows::{Win32::{UI::{WindowsAndMessaging::{CS_OWNDC}, Input::KeyboardAndMouse::{VK_SPACE, VK_RETURN}}}, s};

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
                // when getMessage = 0 (exit without an error) | -1 (exit with an error)
                break;
            },
            Ok(_msg) => {
                /* do nothing and continue the loop */
                // if let Some(ch) = window.keyboard.char_code_to_char(loc!()) {
                //     input_str.push(ch);
                //     // println!("{}: {}", window::message::_id_to_name(_msg.message), _msg.message);
                //     // println!(r"\/");
                //     // println!("{:?}", _msg);
                // }
                println!("{:?}", window.keyboard.read_key());
                
                // if window.keyboard.key_is_pressed(VK_RETURN.0) {
                //     println!("input: {} |\n", input_str);
                //     input_str = "".to_string();
                // }
                // else if window.keyboard.char_buffer.is_some() {
                //     input_str.push(window.keyboard.char_code_to_char(loc!()).unwrap_or('\x00'));
                // }
            },
        }
    }

    // print the exit codes
    window.get_exit_codes();
}
