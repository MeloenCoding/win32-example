use windows::Win32::Foundation::POINTS;
const MAX_BUFFER_SIZE: usize = 16;
const WHEEL_DELTA: i16 = 120;

pub struct Mouse {
    pub event_queue: Vec<MouseEvent>,
    pub is_in_window: bool,
    pub wheel_delta_carry: i16,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub wheel_pressed: bool,
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Copy, Clone)]
pub struct MouseEvent {
    pub mouse_state: MouseState,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub wheel_pressed: bool,
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MouseState {
    LPress,
    LRelease,
    RPress,
    RRelease,
    WheelUp,
    WHeelDown,
    WheelPress,
    WHeelRelease,
    Move,
    Enter,
    Leave,
}

impl Mouse {
    pub fn reset(&mut self) {
        self.event_queue = vec![];
    }

    pub fn on_wheel_delta(&mut self, x: i16, y: i16, delta: i16) {
        self.wheel_delta_carry += delta;

        while self.wheel_delta_carry >= WHEEL_DELTA {
            self.wheel_delta_carry -= WHEEL_DELTA;
            self.on_wheel_up(x, y);
        }

        while self.wheel_delta_carry <= -WHEEL_DELTA {
            self.wheel_delta_carry += WHEEL_DELTA;
            self.on_wheel_down(x, y);
        }
    }

    fn on_wheel_up(&mut self, x: i16, y: i16) {
        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::WheelUp,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x,
            y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    fn on_wheel_down(&mut self, x: i16, y: i16) {
        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::WHeelDown,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x,
            y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_left_press(&mut self) {
        self.left_pressed = true;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::LPress,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_right_press(&mut self) {
        self.right_pressed = true;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::RPress,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_left_release(&mut self) {
        self.left_pressed = false;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::LRelease,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_wheel_press(&mut self) {
        self.wheel_pressed = true;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::WheelPress,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_wheel_release(&mut self) {
        self.wheel_pressed = false;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::WheelUp,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_right_release(&mut self) {
        self.right_pressed = false;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::RRelease,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_mouse_move(&mut self, points: POINTS) {
        self.x = points.x;
        self.y = points.y;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::Move,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_mouse_leave(&mut self) {
        self.is_in_window = false;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::Leave,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn on_mouse_enter(&mut self) {
        self.is_in_window = true;

        self.event_queue.push(MouseEvent {
            mouse_state: MouseState::Enter,
            left_pressed: self.left_pressed,
            right_pressed: self.right_pressed,
            x: self.x,
            y: self.y,
            wheel_pressed: self.wheel_pressed,
        });

        trim_buffer(&mut self.event_queue);
    }

    pub fn get_pos(&self) -> POINTS {
        POINTS {
            x: self.x,
            y: self.y,
        }
    }

    pub fn read(&mut self) -> Option<MouseEvent> {
        if !self.event_queue.is_empty() {
            let e: Option<MouseEvent> = Some(self.event_queue.last().unwrap().to_owned());
            self.event_queue.remove(0);
            return e;
        }
        None
    }
}

fn trim_buffer<T>(buffer: &mut Vec<T>) {
    while buffer.len() > MAX_BUFFER_SIZE {
        buffer.remove(0);
    }
}
