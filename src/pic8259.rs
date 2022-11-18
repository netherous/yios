#![allow(dead_code)]
use x86_64::instructions::port::Port;
const PIC1:u8 = 0x20;//IO base address for master PIC
const PIC2:u8 = 0xA0;//IO base address for slave PIC
const PIC_EOI:u8 = 0x20;//end of interrupt command code
const PIDC_INIT:u8 = 0x11; 


const ICW4_8086:u8 = 0x01;//run in (MCS-80/85) mode 

struct Pic{
    vec_offset: u8,
    command:Port<u8>,
    data:Port<u8>
}

impl Pic{
    const unsafe fn new(vec_offset: u8, cmd: u8, dt: u8)->Pic{
        Pic{
            vec_offset,
            command: Port::new(cmd as u16),
            data: Port::new(dt as u16) 
        }
    }
    //read mask from data port
    unsafe fn read_mask(&mut self)-> u8{
        self.data.read()
    }
    //set_mask in data port
    unsafe fn set_mask(&mut self, irqline:u8){
        let value = self.read_mask() & !(1<<irqline);
        self.data.write(value);
    }
    unsafe fn end_of_interrupt(&mut self){
        self.command.write(PIC_EOI);
    }
}

pub struct ATPic{
    arr_pic: [Pic;2]
}

impl ATPic{
    pub const unsafe fn new(offset1:u8 , offset2:u8)-> ATPic{
        ATPic { 
            arr_pic: [
                Pic::new(offset1, PIC1, PIC1+1),
                Pic::new(offset2, PIC2, PIC2+1)
            ]
        }
    }
    //initiallized both pic  
    pub unsafe fn initialize(&mut self){
        //IO wait setup
        let mut waitport: Port<u8> = Port::new(0x80);
        let mut wait = || waitport.write(0);
        let a1 = self.arr_pic[0].read_mask();
        let a2 = self.arr_pic[1].read_mask();
        self.arr_pic[0].command.write(PIDC_INIT);
        wait();
        self.arr_pic[1].command.write(PIDC_INIT);
        wait();

        self.arr_pic[0].data.write(self.arr_pic[0].vec_offset);
        wait();
        self.arr_pic[1].data.write(self.arr_pic[1].vec_offset);
        wait();

        self.arr_pic[0].data.write(4);
        wait();
        self.arr_pic[1].data.write(2);
        wait();

        self.arr_pic[0].data.write(ICW4_8086);
        wait();
        self.arr_pic[1].data.write(ICW4_8086);
        wait();

        self.arr_pic[0].data.write(a1);
        self.arr_pic[1].data.write(a2);
    }
    //set mask based on irq, but Will need to change later
    pub unsafe fn irq_set_mask(&mut self,mut irqline: u8){
        let ind;
        if irqline < self.arr_pic[1].vec_offset{
            ind = 0; 
        }else{
            ind = 1;
            irqline -= 8;
        }
        self.arr_pic[ind].set_mask(irqline);
    }
    //handle when end of interrupt in send to the PIC, this funtionis called by handler after 
    //handling the interrupts
    pub unsafe fn handle_eoi(&mut self, irqline: u8){
        let ind;
        if irqline < self.arr_pic[1].vec_offset{
            ind = 0; 
        }else{
            ind = 1;
        }
        self.arr_pic[ind].end_of_interrupt();
    }
}
