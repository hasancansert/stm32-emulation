// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell; // enables interior mutability
use std::rc::Rc; // Rc is a reference-counting pointer used for shared ownership data

use crate::ext_devices::{ExtDevices, ExtDevice};
use crate::system::System;
use super::Peripheral;

#[derive(Default)] // default trait for the usart struct allowing an instance to be instantiated with default values
pub struct Usart { 
    pub name: String, //holds the identifier of the USART
    pub ext_device: Option<Rc<RefCell<dyn ExtDevice<(), u8>>>>, // an optional reference-counted pointer to an external device with interior mutability
}

impl Usart {
    pub fn new(name: &str, ext_devices: &ExtDevices) -> Option<Box<dyn Peripheral>> { //implements a constructor
        if name.starts_with("USART") {// if name starts with USART it proceeds to connect a corresponding external device
            let ext_device = ext_devices.find_serial_device(&name);//attempts to find a serial USART device
            let name = ext_device.as_ref()  //attempts to connect Usart instance to this device and updates the 'name'
                .map(|d| d.borrow_mut().connect_peripheral(name))
                .unwrap_or_else(|| name.to_string());
            Some(Box::new(Self { name, ext_device, ..Default::default() })) // wraps the newly created USART instance in a box(a heap-allocated pointer) and returns if the name is valid
        } else {
            None
        }
    }
    
//    pub fn send_data(&mut self, data:u8){
//        if let Some(ref ext_device) = self.ext_device{
//            ext_device.borrow_mut().write(&System::default(), (), data);
//            trace!("{} receive_data={:02x}", self.name, data);
//            data
//        }else{
//            0
//        }
//        
//        
//        
//    }
//    
//    
//    pub fn receive_data(&mut self) -> u8{
//        if let Some(ref ext_device) = self.ext_device{
//            let data = ext_device.borrow_mut().read(&System::default(), ());
//            trace!("{} receive_data={:02x}", self.name, data);
//            data
//        }else{
//            0
//        }
//        
//        
//    }
//    
    
}

impl Peripheral for Usart {
    fn read(&mut self, sys: &System, offset: u32) -> u32 { //defines a read method required by the peripheral
        match offset {
            0x0000 => {
                // SR register
                // Bit 7 TXE: Transmit data register empty
                // Bit 6 TC: Transmission complete
                // Bit 5 RXNE: Read data register not empty
                // Bit 4 IDLE: IDLE line detected
                // We could do something smarter to indicate that there's data to read
                (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4)
            }
            0x0004 => { //it interprets this as a read from Data Register
                // DR register
                let v = self.ext_device.as_ref().map(|d| 
                    d.borrow_mut().read(sys, ()) // it reads data from the connected external device
                ).unwrap_or_default() as u32; // unwrap_or_default call ensures a default value (0) is returned if no device is connected

                trace!("{} read={:02x}", self.name, v);
                v
            }
            _ => 0
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) { //defines a write method required by the peripheral
        match offset {
            0x0004 => { //write to data register
                // DR register
                self.ext_device.as_ref().map(|d|
                    d.borrow_mut().write(sys, (), value as u8) // code attempts to write provided value to the connected external device
                );

                trace!("{} write={:02x}", self.name, value as u8); // it logs the write operation
            }
            _ => {} //for any offset write function has no action.
        }
    }
}
