use self::scancode_set::ScancodeSet1;

///General design pattern
///the keyboard is assume to use scancode set 1, and US qwert 104 english 
///keyboard layout.
///
///key_event is generated from scancode input which contains the status
///of the key (pressed or released) and the key 
///
///Usage pattern
///takes in scancode from pic, then generate key_event
///this is better because key_event could contain informations of
///if the key is pressed or released, and by generating key_event,
///key_event contains key state and scancode
///
///Decode/Mapping of key_event, each scancode will be mapped to a 
///key_code, then keycode will be decoded into possible ascii or rawcode 
///characters.

//layout mod to represent US qwert 104 english keyboard 
pub mod layout;
use layout::Us104;

//scancode set mod to represent the scancode set 1
pub mod scancode_set;


//Keyboard struct that contains all informations 
pub struct Keyboard{
    lay: Us104,
    sc_set: ScancodeSet1,
}

impl Keyboard{
    pub fn new()->Keyboard{
        Keyboard { lay: Us104{}, sc_set: ScancodeSet1{} }
    }

    pub fn read_byte(&self, code : u8)->DecodeKey{
        if code > 0x58{
            return DecodeKey::Rawcode(code);
        }
        let kc = self.sc_set.map_keycodee(code);
        Us104::map_to_decode_key(kc)
    }
}

//Keyboard errors that need be considered
pub enum Error{
    UnknownKeyCode,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DecodeKey{
    Ascii(char),
    Rawcode(u8),
} 


//Keycodes that is generated from the keyboard
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[repr(u8)]
pub enum KeyCode{
    AltLeft = 0,
    AltRight = 1,
    ArrowDown = 2,
    ArrowLeft = 3,
    ArrowRight = 4,
    ArrowUp = 5,
    BackSlash = 6,
    Backspace = 7,
    BackTick = 8,
    BracketSquareLeft = 9,
    BracketSquareRight = 10,
    Break = 11,
    CapsLock = 12,
    Comma = 13,
    ControlLeft = 14,
    ControlRight = 15,
    Delete = 16,
    End = 17,
    Enter = 18,
    Escape = 19,
    Equals = 20,
    F1 = 21,
    F2 = 22,
    F3 = 23,
    F4 = 24,
    F5 = 26,
    F6 = 27,
    F7 = 28,
    F8 = 29,
    F9 = 30,
    F10 = 31,
    F11 = 32,
    F12 = 33,
    Fullstop = 34,
    Home = 36,
    Insert = 37,
    Key1 = 38,
    Key2 = 39,
    Key3 = 40,
    Key4 = 41,
    Key5 = 42,
    Key6 = 43,
    Key7 = 44,
    Key8 = 46,
    Key9 = 47,
    Key0 = 48,
    Menus = 49,
    Minus = 50,
    Numpad0 = 51,
    Numpad1 = 52,
    Numpad2 = 53,
    Numpad3 = 54,
    Numpad4 = 56,
    Numpad5 = 57,
    Numpad6 = 58,
    Numpad7 = 59,
    Numpad8 = 60,
    Numpad9 = 61,
    NumpadEnter = 62,
    NumpadLock = 63,
    NumpadSlash = 64,
    NumpadStar = 66,
    NumpadMinus = 67,
    NumpadPeriod = 68,
    NumpadPlus = 69,
    PageDown = 70,
    PageUp = 71,
    PauseBreak = 72,
    PrintScreen = 73,
    ScrollLock = 74,
    SemiColon = 76,
    ShiftLeft = 77,
    ShiftRight = 78,
    Slash = 79,
    Spacebar = 80,
    SysReq = 81,
    Tab = 82,
    Quote = 83,
    WindowsLeft = 84,
    WindowsRight = 86,
    A = 87,
    B = 88,
    C = 89,
    D = 90,
    E = 91,
    F = 92,
    G = 93,
    H = 94,
    I = 96,
    J = 97,
    K = 98,
    L = 99,
    M = 100,
    N = 101,
    O = 102,
    P = 103,
    Q = 104,
    R = 106,
    S = 107,
    T = 108,
    U = 109,
    V = 110,
    W = 111,
    X = 112,
    Y = 113,
    Z = 114,
    HashTilde = 115,
    PrevTrack = 116,
    NextTrack = 117,
    Mute = 118,
    Calculator = 119,
    Play = 120,
    Stop = 121,
    VolumeDown = 122,
    VolumeUp = 123,
    WWWHome = 124,
    PowerOnTestOk = 125,
    Oem102 = 126,
    PrintScreen2 = 127,
    TooManyKeys = 128,
    Error,
}

#[derive(Debug,PartialEq, Eq,Clone, Copy)]
pub enum KeyState{
    Pressed,
    Released,
}


pub struct KeyEvent{}

//implement for future purpose
pub struct Modifiers{}

//implement for future purose, to enact sign INT
pub enum CtrlHandle{}
