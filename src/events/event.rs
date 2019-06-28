use termion::event::Key;

#[derive(Debug, Clone)]
pub enum NoteMessage {
    On = 0x90,
    Off = 0x80,
}

#[derive(Debug, Clone)]
pub enum Event {
    Note {
        message: NoteMessage,
        note: u8,
        velocity: u8,
    },
    Key {
        key: Key,
    },
    Pause,
    Quit,
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
}
