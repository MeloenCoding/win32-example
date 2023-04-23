use super::error::{self, CallLocation};

#[derive(Debug)]
pub struct Keyboard {
    pub key_type: KeyType,
    pub key_state: KeyState,
    pub key_code: Option<u32>,
    pub char_code: Option<u32>
}

#[derive(Debug)]
pub enum KeyType {
    SysKey,
    Key,
    Idle
}

#[derive(Debug)]
pub enum KeyState {
    Press,
    Release,
    Invalid
}


impl Keyboard {
    pub fn handle_key_down(&mut self, key_code: u32, key_type: KeyType) {
        self.key_code = Some(key_code);
        self.key_type = key_type;
        self.key_state = KeyState::Press;
    }

    pub fn handle_key_up(&mut self) {
        self.key_state = KeyState::Release;
    }

    pub fn handle_char(&mut self, char_code: u32) {
        self.char_code = Some(char_code);
    }

    pub fn char_code_to_char(&self, origin: CallLocation) -> Result<char, ()> {
        let code: Option<u32> = self.char_code;
        if code.is_none() {
            return Err(());
        }
        return Ok(char::from_u32(code.unwrap()).unwrap_or_else(|| {
            error::WindowError::new("Unable to convert u32 char to normal char.", None, origin)
        }));
    }
}