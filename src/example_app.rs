use std::time::SystemTime;

use windows::Win32::UI::{Input::KeyboardAndMouse::VK_RETURN, WindowsAndMessaging::CS_OWNDC};

use crate::window::Window;

pub struct ExampleApp<'a> {
    pub window: Window<'a>,
    input_buffer: String,
    time_buffer: SystemTime,
}

impl ExampleApp<'_> {
    pub fn create() -> ExampleApp<'static> {
        let app = ExampleApp {
            window: Window::new("Example App", CS_OWNDC, 1000, 750),
            input_buffer: String::new(),
            time_buffer: SystemTime::now(),
        };
        app.window.show_window();
        return app;
    }

    pub fn launch(&mut self) -> usize {
        let mut exit_code: Option<usize>;
        loop {
            exit_code = self.window.handle_messages();
            if exit_code.is_some() {
                break;
            }
            self.render_frame();
        }
        return exit_code.unwrap();
    }

    pub fn render_frame(&mut self) {
        // A test to check if the window updates even if there are no events:
        let time_alive: std::time::Duration =
            SystemTime::now().duration_since(self.time_buffer).unwrap();

        let elapsed_time: windows::core::PCSTR = windows::core::PCSTR::from_raw(
            format!(
                "{},{}s",
                time_alive.as_secs(),
                time_alive.as_millis() % 1000
            )
            .as_ptr(),
        );
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SetWindowTextA(self.window.hwnd, elapsed_time)
        };

        // App logic
        if let Some(ch) = self.window.keyboard.read_char() {
            self.input_buffer.push(ch);
        }

        if self.window.keyboard.key_is_pressed_pop(VK_RETURN.0) {
            println!("{:?}", self.input_buffer);
            self.input_buffer = "".to_string();
        }
    }
}
