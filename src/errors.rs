use std::process::exit;

#[derive(Debug)]
pub struct Error(pub i32, pub String);
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::errors::Result::Err($crate::errors::Error(1, format_args!($($arg)*).to_string()))
    };
    ($code:literal => $($arg:tt)*) => {
        $crate::errors::Result:Err($crate::errors::Error ($code, format_args!($($arg)*).to_string()))
    };
}

pub fn handle_error(e: Error) -> ! {
    eprintln!("ketch: {}", e.1);
    exit(e.0);
}
