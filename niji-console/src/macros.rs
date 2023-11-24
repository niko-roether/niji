#[macro_export]
macro_rules! prompt {
	(default: $default:expr, $($arg:tt)+) => {
        $crate::__private_api::prompt(&format_args!($($arg)+), Some($default)).unwrap()
    };
    ($($arg:tt)+) => {
        $crate::__private_api::prompt(&format_args!($($arg)+), None).unwrap()
    }
}
