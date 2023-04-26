const MAX_BUFFER_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub struct Keyboard {
    pub key_states: Option<Vec<u8>>,
    
    pub key_buffer: Option<Vec<KeyEvent>>,
    pub char_buffer: Option<Vec<char>>,
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
    /// Reset buffers and keystate bitmap
    pub fn reset(&mut self) {
        self.key_states = Some(vec![0; 256]);
        self.key_buffer = Some(vec![]);
        self.char_buffer = Some(vec![])
    }

    /// Check if key is pressed but don't remove it from the KeyEvent queue. See key_is_pressed_clear()
    pub fn key_is_pressed(&self, target_key: u16) -> bool {
        return self.key_states.as_ref().unwrap()[target_key as usize] == 1;
    }

    /// Check if key is pressed and remove it from the KeyEvent queue. See key_is_pressed()
    pub fn key_is_pressed_clear(&mut self, target_key: u16) -> bool {
        let key_state = self.key_states.as_ref().unwrap()[target_key as usize] == 1;
        self.key_states.as_mut().unwrap()[target_key as usize] = 0;
        return key_state;
    }

    /// Read key from the queue and remove it 
    pub fn read_key(&mut self) -> Option<KeyEvent> {
        if !self.key_buffer.as_mut().unwrap().is_empty() {
            let e: KeyEvent = self.key_buffer.as_mut().unwrap().last().unwrap().to_owned();
            self.key_buffer.as_mut().unwrap().remove(0);
            return Some(e);
        }
        return None;
    }

    /// Read char from the queue and remove it
    pub fn read_char(&mut self) -> Option<char> {
        
        if !self.char_buffer.as_mut().unwrap().is_empty() {
            let ch: Option<char> = Some(self.char_buffer.as_mut().unwrap().last().unwrap().to_owned());
            self.char_buffer.as_mut().unwrap().remove(0);
            return ch;
        }
        return None;
    }

    pub fn clear_key_queue(&mut self) {
        self.key_buffer = Some(vec![]);
    }

    pub fn clear_char_queue(&mut self) {
        self.char_buffer = Some(vec![]);
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
        self.key_buffer.as_mut().unwrap().push(KeyEvent { key_state: KeyState2::Press, key_code });
        trim_buffer(&mut self.key_buffer.as_mut().unwrap());
    }

    pub fn on_key_release(&mut self, key_code: u32) {
        self.key_states.as_mut().unwrap()[key_code as usize] = 0;
        self.key_buffer.as_mut().unwrap().push(KeyEvent { key_state: KeyState2::Release, key_code });
        trim_buffer(&mut self.key_buffer.as_mut().as_mut().unwrap());
    }

    pub fn on_char(&mut self, char_code: u32) {
        self.char_buffer.as_mut().unwrap().push(char::from_u32(char_code).unwrap());
        trim_buffer(&mut self.char_buffer.as_mut().as_mut().unwrap());
    }

    pub fn key_queue_is_empty(&self) -> bool {
        return self.key_buffer.as_ref().unwrap().is_empty();
    }

    pub fn char_queue_is_empty(&self) -> bool {
        return self.char_buffer.as_ref().unwrap().is_empty();
    }

}

pub fn trim_buffer<T>(buffer: &mut Vec<T>) {
    while buffer.len() > MAX_BUFFER_SIZE {
        buffer.remove(0);
    }
}