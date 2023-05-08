use windows::{
    s,
    Win32::{
        UI::{Input::KeyboardAndMouse::VK_RETURN, WindowsAndMessaging::CS_OWNDC},
    },
};

use crate::{window::Window};

pub struct ExampleApp<'a> {
    pub window: Window<'a>,
    input_buffer: String,
}

impl ExampleApp<'_> {
    pub fn create() -> ExampleApp<'static> {
        let app = ExampleApp {
            window: Window::new(s!("Example App"), CS_OWNDC, 1000, 750),
            input_buffer: String::new(),
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
