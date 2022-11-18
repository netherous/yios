pub struct Us104;
use super::{KeyCode,DecodeKey};

impl Us104{
    pub fn map_to_decode_key(&self, keyCode: KeyCode)-> DecodeKey{
        use crate::print;
        let cc : u8 = keyCode as u8;      
        let mut i:u8 = 0;
        //numbers
        for c in 38..48{
           i+=1;
           if c == cc{
               return DecodeKey::Ascii((i+48)as char);
           }
        }
        let mut i:u8 = 97;
        for c in 87..115{
            if cc == c{
                return DecodeKey::Ascii(i as char);
            }
            i+=1;
        }

        //letters

        //punctuation and enter
        
        //return default
        return DecodeKey::Rawcode(keyCode as u8);
    }
}
