const KEY_NUM_TYPES: usize = 5;

pub(crate) enum Key {
    Left,
    Up,
    Right,
    Down,
    Space,
}

fn key_to_int(key: Key) -> usize {
    match key {
        Key::Left => 0,
        Key::Up => 1,
        Key::Right => 2,
        Key::Down => 3,
        Key::Space => 4,
    }
}

pub(crate) struct Input {
    keys: [u32; KEY_NUM_TYPES],
}

impl Input {
    pub fn new() -> Input {
        Input { keys: [0; 5] }
    }
    pub fn press(&mut self, key: Key) {
        let i = key_to_int(key);
        if self.keys[i] == 0 {
            self.keys[i] = 1;
        }
    }
    pub fn release(&mut self, key: Key) {
        self.keys[key_to_int(key)] = 0;
    }
    pub fn tick(&mut self) {
        for i in 0..KEY_NUM_TYPES {
            if self.keys[i] > 0 {
                self.keys[i] += 1;
            }
        }
    }
    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys[key_to_int(key)] > 0
    }
    pub fn get(&self, key: Key) -> u32 {
        self.keys[key_to_int(key)]
    }
}
