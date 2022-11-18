///General design pattern
///the keyboard is assume to use scancode set 1, and US qwert 104 english 
///keyboard layout.
///
///key_event is generated from scancode input which contains the status
///of the key (pressed or released) and the key 
///
///Usage pattern
///takes in scancode from scane code set 1, then generate key_event
///this is better because key_event could contain informations of
///if the key is pressed or released, and by generating key_event
///could detect errors possibly coming from scancode
///
///Decode/Mapping of key_event, each key_event will be mapped to a 
///key_code, then keycode will be decoded into possible ascii or unicode 
///characters.

//layout mod to represent US qwert 104 english keyboard 
pub mod layout;

//scancode set mod to represent the scancode set 1
pub mod scancode_set;


//Keyboard struct that contains all informations 
pub struct Keyboard{
}

//Keyboard errors that need be considered
pub enum Error{}

//Keycodes that is generated from the keyboard
pub enum KeyCode{}

pub enum KeyState{}

pub enum CtrlHandle{}

pub struct KeyEvent{}

pub struct Modifiers{}

pub enum DecodedKey{}
