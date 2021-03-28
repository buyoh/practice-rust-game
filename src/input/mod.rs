use std::sync::mpsc::{channel, Receiver, Sender};

const KEY_NUM_TYPES: usize = 5;

pub enum Key {
    Left,
    Up,
    Right,
    Down,
    // Space,
}

fn key_to_int(key: Key) -> usize {
    match key {
        Key::Left => 0,
        Key::Up => 1,
        Key::Right => 2,
        Key::Down => 3,
        // Key::Space => 4,
    }
}

#[derive(Clone)]
pub struct InputInfo {
    pub keys: [bool; KEY_NUM_TYPES], // note: we may consider the pressure.
    pub triggers: [bool; KEY_NUM_TYPES],
}

impl InputInfo {
    pub fn new() -> InputInfo {
        InputInfo {
            keys: [false; KEY_NUM_TYPES],
            triggers: [false; KEY_NUM_TYPES],
        }
    }
    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys[key_to_int(key)]
    }

    pub fn update(&mut self, input: InputDriverInputInfo) {
        for i in 0..KEY_NUM_TYPES {
            self.keys[i] = input.keys[i];
            self.triggers[i] |= input.keys[i];
        }
    }

    pub fn down_trigger(&mut self) {
        self.triggers.fill(false);
    }
}

pub struct Input {
    input_info: InputInfo,
    input_driver_rx: Receiver<InputDriverInputInfo>,
}

impl Input {
    pub fn create_with_driver() -> (Input, InputDriver) {
        let (tx, rx) = channel::<InputDriverInputInfo>();
        (
            Input {
                input_info: InputInfo::new(),
                input_driver_rx: rx,
            },
            InputDriver::new(tx),
        )
    }

    pub fn owned_input(&self) -> InputInfo {
        self.input_info.clone()
    }

    pub fn down_trigger(&mut self) {
        self.input_info.down_trigger();
    }

    pub fn procedure_events(&mut self) {
        for info in self.input_driver_rx.try_iter() {
            self.input_info.update(info);
        }
    }
}

#[derive(Clone)]
pub struct InputDriverInputInfo {
    // pressure の実装を考慮して、毎回全てのデータを送る
    pub keys: [bool; KEY_NUM_TYPES],
}

impl InputDriverInputInfo {
    pub fn new() -> InputDriverInputInfo {
        InputDriverInputInfo {
            keys: [false; KEY_NUM_TYPES],
        }
    }
}

pub struct InputDriver {
    sender: Sender<InputDriverInputInfo>,
    input_info: InputDriverInputInfo,
}

impl InputDriver {
    pub fn new(sender: Sender<InputDriverInputInfo>) -> InputDriver {
        InputDriver {
            sender: sender,
            input_info: InputDriverInputInfo::new(),
        }
    }

    pub fn handle_key_press_event(&mut self, key: gdk::keys::Key) {
        if let Some(ikey) = convert_gdk_key(key) {
            self.input_info.keys[key_to_int(ikey)] = true;
            // ignore failure
            let _ = self.sender.send(self.input_info.clone());
        }
    }

    pub fn handle_key_release_event(&mut self, key: gdk::keys::Key) {
        if let Some(ikey) = convert_gdk_key(key) {
            self.input_info.keys[key_to_int(ikey)] = false;
            // ignore failure
            let _ = self.sender.send(self.input_info.clone());
        }
    }
}

fn convert_gdk_key(key: gdk::keys::Key) -> Option<Key> {
    // match key {
    //     gdk::keys::constants::leftarrow => self.key_left = true,
    //     gdk::keys::constants::uparrow => self.key_up = true,
    //     gdk::keys::constants::rightarrow => self.key_right = true,
    //     gdk::keys::constants::downarrow => self.key_down = true,
    //     _ => (),
    // };
    // TODO: 妥協策
    match key.name().unwrap().as_str() {
        "Left" => Some(Key::Left),
        "Up" => Some(Key::Up),
        "Right" => Some(Key::Right),
        "Down" => Some(Key::Down),
        _ => None,
    }
}
