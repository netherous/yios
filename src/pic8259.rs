#![allow(dead_code)]
use x86_64::instructions::port::Port;

const PIC1:u8 = 0x20;//IO base address for master PIC
const PIC2:u8 = 0xA0;//IO base address for slave PIC
const PIC_EOI:u8 = 0x20;//end of interrupt command code
const PIDC_INIT:u8 = 0x11; 

const ICW4_8086:u8 = 0x01;

struct Pic{
    vec_offset: u8,
    command:Port<u8>,
    data:Port<u8>
}

impl Pic{
    pub fn new(vec_offset: u8, cmd: u8, dt: u8)->Pic{
        Pic{
            vec_offset,
            command: Port::new(cmd as u16),
            data: Port::new(dt as u16) 
        }
    }
}

pub struct ATPic{
    arr_pic: [Pic;2]
}

impl ATPic{
    pub fn new(offset1:u8 , offset2:u8)-> ATPic{
        ATPic { 
            arr_pic: [
                Pic::new(offset1, PIC1, PIC1+1),
                Pic::new(offset2, PIC2, PIC2+1)
            ]
        }
    }
    
    pub fn initialize(){
        todo!()
    }
}
