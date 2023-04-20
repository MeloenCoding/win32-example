pub struct Keyboard {
    pub types: Option<KeyType>,
    pub state: Option<KeyState>,
    pub key: Option<u32>
}

pub enum KeyType {
    SysKey,
    Key,
    Idle
}

pub enum KeyState {
    Press,
    Release,
    Invalid
}

impl Keyboard {
    pub fn new() -> Keyboard{
        Keyboard { types: None, state: None, key: None }
    }
}