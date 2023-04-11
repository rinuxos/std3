#![allow(unreachable_patterns)]

use uart_16550::SerialPort;

pub unsafe fn write(port: u16, data: u8) -> bool {
    let mut serial_port = SerialPort::new(port);
    serial_port.init();
    match serial_port.send(data) {
        () => true,
        _ => false
    }
}

pub unsafe fn read(port: u16) -> (u8,bool) {
    let mut serial_port = SerialPort::new(port);
    serial_port.init();
    match serial_port.receive() {
        v => (v,true),
        _ => (0,false)
    }
}