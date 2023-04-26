const MAX_BUFFER_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub struct Keyboard {
    /// A map of all the keys represented as 0, 1. 0 means key is up and 1 means key is up
    pub key_states: Vec<u8>,

    /// A FIFO (First In First Out) list of all the recent [KeyEvent]'s.
    /// These events are:
    /// - [WM_KEYUP](windows::Windows::Win32::UI::WindowsAndMessaging::WM_KEYUP)
    /// - [WM_SYSKEYUP](const@windows::Windows::Win32::UI::WindowsAndMessaging::WM_SYSKEYUP)
    /// - [WM_KEYDOWN](const@windows::Windows::Win32::UI::WindowsAndMessaging::WM_KEYDOWN)
    /// - [WM_SYSKEYDOWN](const@windows::Windows::Win32::UI::WindowsAndMessaging::WM_SYSKEYDOWN)
    pub key_queue: Vec<KeyEvent>,

    /// A FIFO (First In First Out) list of all the recent [WM_CHAR][ch] [KeyEvent]'s.
    /// 
    /// [ch]: windows::Windows::Win32::UI::WindowsAndMessaging::WM_CHAR
    pub char_queue: Vec<char>,

    /// 
    pub auto_repeat_enabled: bool
}

#[derive(Debug, Copy, Clone)]
pub struct KeyEvent {
    key_state: KeyState,
    key_code: u32
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyState {
    Press,
    Release
}

#[derive(Debug,PartialEq)]
pub enum KeyType {
    SysKey,
    Key,
    Idle
}

impl Keyboard {
    /// Reset buffers and keystate bitmap
    pub fn reset(&mut self) {
        self.key_states = vec![0; 256];
        self.key_queue = vec![];
        self.char_queue = vec![];
    }

    /// Check if key is pressed but don't remove it from the KeyEvent queue. See key_is_pressed_clear()
    pub fn key_is_pressed(&self, target_key: u16) -> bool {
        return self.key_states[target_key as usize] == 1;
    }

    /// Check if key is pressed and remove it from the [KeyEvent] queue. 
    /// If you don't want to remove the key, See [key_is_pressed()]
    pub fn key_is_pressed_clear(&mut self, target_key: u16) -> bool {
        let key_state = self.key_states[target_key as usize] == 1;
        self.key_states[target_key as usize] = 0;
        return key_state;
    }

    /// Get the [KeyEvent] from the 
    pub fn read_key(&mut self) -> Option<KeyEvent> {
        if !self.key_queue.is_empty() {
            let e: KeyEvent = self.key_queue.last().unwrap().to_owned();
            self.key_queue.remove(0);
            return Some(e);
        }
        return None;
    }

    /// Read [char] from the queue and remove it
    pub fn read_char(&mut self) -> Option<char> {
        
        if !self.char_queue.is_empty() {
            let ch: Option<char> = Some(self.char_queue.last().unwrap().to_owned());
            self.char_queue.remove(0);
            return ch;
        }
        return None;
    }

    pub fn clear_key_queue(&mut self) {
        self.key_queue = vec![];
    }

    pub fn clear_char_queue(&mut self) {
        self.char_queue = vec![];
    }

    pub fn clear_all_queues(&mut self) {
        self.clear_char_queue();
        self.clear_key_queue();
    }

    pub fn disable_auto_repeat(&mut self) {
        self.auto_repeat_enabled = false;
    }

    pub fn enable_auto_repeat(&mut self) {
        self.auto_repeat_enabled = true;
    }

    pub fn on_key_press(&mut self, key_code: u32) {
        self.key_states[key_code as usize] = 1;
        self.key_queue.push(KeyEvent { key_state: KeyState::Press, key_code });
        trim_buffer(&mut self.key_queue);
    }

    pub fn on_key_release(&mut self, key_code: u32) {
        self.key_states[key_code as usize] = 0;
        self.key_queue.push(KeyEvent { key_state: KeyState::Release, key_code });
        trim_buffer(&mut self.key_queue.as_mut());
    }

    pub fn on_char(&mut self, char_code: u32) {
        self.char_queue.push(char::from_u32(char_code).unwrap());
        trim_buffer(&mut self.char_queue.as_mut());
    }

    pub fn key_queue_is_empty(&self) -> bool {
        return self.key_queue.is_empty();
    }

    pub fn char_queue_is_empty(&self) -> bool {
        return self.char_queue.is_empty();
    }

}

pub fn trim_buffer<T>(buffer: &mut Vec<T>) {
    while buffer.len() > MAX_BUFFER_SIZE {
        buffer.remove(0);
    }
}