use super::error::{self, CallLocation};

const MAX_BUFFER_SIZE: usize = 16;

#[derive(Debug)]
pub struct Keyboard {
    pub key_type: KeyType,
    pub key_state: KeyState,
    pub key_buffer: Option<u32>,
    pub char_buffer: Option<u32>,

    pub key_states: Option<Vec<u8>>,
    pub key_buffer2: Option<Vec<KeyEvent>>,
    pub char_buffer2: Option<Vec<char>>,
    pub auto_repeat_enabled: bool
}

#[derive(Debug, Copy, Clone)]
pub struct KeyEvent {
    key_state: KeyState2,
    key_code: u32
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyState2 {
    Press,
    Release
}

#[derive(Debug,PartialEq)]
pub enum KeyType {
    SysKey,
    Key,
    Idle
}

#[derive(Debug,PartialEq)]
pub enum KeyState {
    Press,
    Release,
    Invalid
}


impl Keyboard {
    // old and very buggy stuff
    pub fn reset(&mut self) {
        self.key_states = Some(vec![0; 256]);
    }

    pub fn clear(&mut self) {
        self.char_buffer = None;
        self.key_buffer = None;
        self.key_states = Some(vec![0; 256]);
        self.key_state = KeyState::Invalid;
        self.key_type = KeyType::Idle;
    }

    pub fn handle_key_down(&mut self, key_buffer: u32, key_type: KeyType) {
        self.key_states.as_deref_mut().unwrap()[key_buffer as usize] = 1;

        self.key_buffer = Some(key_buffer);
        self.key_type = key_type;
        self.char_buffer = None;
        self.key_state = KeyState::Press;

        // println!("{:?}", self.key_states);
    }

    pub fn handle_key_up(&mut self) {
        self.key_state = KeyState::Release;
        self.key_states.as_deref_mut().unwrap()[self.key_buffer.unwrap() as usize] = 0;
        // println!("{:?}", self.key_states);
    }

    pub fn handle_char(&mut self, char_buffer: u32) {
        self.char_buffer = Some(char_buffer);
    }

    pub fn char_code_to_char(&self, origin: CallLocation) -> Option<char> {
        let code: Option<u32> = self.char_buffer;
        if code.is_none() || self.key_state != KeyState::Press {
            return None;
        }
        return Some(char::from_u32(code.unwrap()).unwrap_or_else(|| {
            error::WindowError::new("Unable to convert u32 char to normal char.", None, origin)
        }));
    }

    pub fn key_is_pressed(&self, target_key: u16) -> bool {
        if self.key_buffer.is_none() || self.key_buffer.unwrap() != target_key as u32 {
            return false;
        }

        return self.key_state == KeyState::Press;
    }

    // new stuff

    pub fn key_is_pressed2(&self, target_key: u16) -> bool {
        return self.key_states.as_ref().unwrap()[target_key as usize] == 1;
    }

    pub fn read_key(&mut self) -> Option<KeyEvent>{
        if !self.key_buffer2.as_mut().unwrap().is_empty() {
            let e: KeyEvent = *self.key_buffer2.as_mut().unwrap().last().unwrap();
            self.key_buffer2.as_mut().unwrap().pop();
            return Some(e);
        }
        return None;
    }

    pub fn key_queue_is_empty(&self) -> bool {
        return self.key_buffer2.as_ref().unwrap().is_empty();
    }

    pub fn char_queue_is_empty(&self) -> bool {
        return self.char_buffer2.as_ref().unwrap().is_empty();
    }

    pub fn read_char(&mut self, origin: CallLocation) -> Option<char> {
        if !self.char_buffer2.as_mut().unwrap().is_empty() {
            return Some(self.char_buffer2.as_mut().unwrap().last().unwrap().to_owned());
        }
        return None;
    }

    pub fn clear_key_queue(&mut self) {
        self.key_buffer2 = Some(vec![]);
    }

    pub fn clear_char_queue(&mut self) {
        self.char_buffer2 = Some(vec![]);
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
        self.key_states.as_mut().unwrap()[key_code as usize] = 1;
        self.key_buffer2.as_mut().unwrap().push(KeyEvent { key_state: KeyState2::Press, key_code });
        trim_buffer(&mut self.key_buffer2.as_mut().unwrap());
    }

    pub fn on_key_release(&mut self, key_code: u32) {
        self.key_states.as_mut().unwrap()[key_code as usize] = 0;
        self.key_buffer2.as_mut().unwrap().push(KeyEvent { key_state: KeyState2::Release, key_code });
        trim_buffer(&mut self.key_buffer2.as_mut().as_mut().unwrap());
    }

    pub fn on_char(&mut self, char_code: u32) {
        self.char_buffer2.as_mut().unwrap().push(char::from_u32(char_code).unwrap())
    }

}

pub fn trim_buffer(buffer: &mut Vec<KeyEvent>) {
    while buffer.len() > MAX_BUFFER_SIZE {
        buffer.pop();
    }
}