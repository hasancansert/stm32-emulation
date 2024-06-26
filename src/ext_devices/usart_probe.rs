// SPDX-License-Identifier: GPL-3.0-or-later

/* 
This code essentially defines a USART probe which can be used to collect incoming data until a newline is received, at which point it logs the collected data and 
clears it's buffer.
The actual sending and receiving functionality would need to be hooked into the system's USART peripheral emulation logic.

The probe could be used for debugging or testing by inspecting the data transferred via USART.

*/

use anyhow::Result; // a library for convenient error handling
use serde::Deserialize; //a framework for serializing and deserializing data

use crate::system::System; //Imports System struct from the system module 'System' is a represantation of emulator's system

use super::ExtDevice; //a trait to define interface for the external device

//'Debug' --> formatting the struct in a way suitable for debugging output
//'Deserialize' --> to allow instances of UsartProbeConfig to be constructed from serialized data (like config file)
//'Default' --> to provide a defaul-zero constructor
#[derive(Debug, Deserialize, Default)] // Derives  3 traits for UsartProbeConfig struct 

pub struct UsartProbeConfig { // This struct holds configuration for the USART probe
    pub peripheral: String,
}

#[derive(Default)]
pub struct UsartProbe {  
    pub config: UsartProbeConfig,//'config'--> holds the configuration of this probe 
    name: String,
    rx: Vec<u8>, //a private field that vector of bytes used as a buffer for incoming data
}
//Implements a constructor for 'UsartProbe' returns a 'Result' that when successfull
//contains a new UsartProbe instance with the provided config and default values for other fields.
impl UsartProbe {
    pub fn new(config: UsartProbeConfig) -> Result<Self> {
        Ok(Self { config, ..Self::default() }) // ???????????
    }
}

//The implementation specifies that no address information ('()') is required and data will be in u8 format
impl ExtDevice<(), u8> for UsartProbe { // This trait defines how external devices interact with the 'System'.
    fn connect_peripheral(&mut self, peri_name: &str) -> String { // defines a method to connect probe to peripheral. It sets the probe name and returns it.
        self.name = format!("{} usart-probe", peri_name); // formats and stores the 'name' field suffixed with usart-probe
        self.name.clone() // returns a copy of the formatted 'name'
    }

    fn read(&mut self, _sys: &System, _addr: ()) -> u8 { // defines a read method required by the ExtDevice trait
        0 // always returns 0 means which could no data available
    }

    fn write(&mut self, _sys: &System, _addr: (), v: u8) { // defines a 'write' method which processed a byte 'v' written to the probe. The '_sys' and '_addr' parameters unusued
        if v == 0x0a {// checks if the incoming byte is a newline character '\n' == ASCII(0x0a)
            // EOL
            let line = String::from_utf8_lossy(&self.rx); //converts accumulated bytes in the 'rx' buffer into a string for logging, using "lossy" conversion
            let line = line.trim();//trims whitespaces from both ends of the string
            info!("{} '{}'", self.name, line); // Logs the collected line with the probe's name to the info level log
            self.rx.clear(); // clears the receive buffer after logging, preparing it for the next line of input
        } else {//This part of the conditional logic decides what to do with incoming bytes.
            self.rx.push(v); // if the byte is not a new line, It's added to the rx buffer, to be part of the next log message.
        }
    }
}
