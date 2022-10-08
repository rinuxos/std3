#![allow(unreachable_patterns)]

use crate::io::prelude::*;

pub unsafe fn write(port: u16, data: u8) -> bool {
    match u8::write_to_port(port, data) {
        () => true,
        _ => false
    }
}

pub unsafe fn read(port: u16) -> (u8,bool) {
    match u8::read_from_port(port) {
        v => (v,true),
        _ => (0,false)
    }
}