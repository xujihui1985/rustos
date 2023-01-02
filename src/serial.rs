use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt::Write;

lazy_static!{
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe {SerialPort::new(0x3F8)}; //0x3F8 is the stand serial port number for the first serial interface
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    SERIAL1.lock().write_fmt(args).expect("failed to write to serial port");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
       $crate::serial::_print(format_args!($($arg)*)) ;
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

