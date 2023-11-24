#[macro_export]
macro_rules! prompt {
	(default: $default:expr, $($arg:tt)+) => {
        $crate::__private_api::prompt(&format_args!($($arg)+), Some($default)).unwrap()
    };
    ($($arg:tt)+) => {
        $crate::__private_api::prompt(&format_args!($($arg)+), None).unwrap()
    }
}

#[macro_export]
macro_rules! heading {
    ($($arg:tt)+) => {
        $crate::__private_api::heading(&format_args!($($arg)+)).unwrap()
    }
}

#[macro_export]
macro_rules! println {
	($($arg:tt)+) => {
		$crate::__private_api::println(Some(&format_args!($($arg)+))).unwrap()
	};
    () => {
        $crate::__private_api::println(None).unwrap()
    }
}
