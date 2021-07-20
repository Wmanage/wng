#[derive(Debug)]
pub struct Error(pub String);
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::errors::Result::Err($crate::errors::Error(format_args!($($arg)*).to_string()))
    };
}
