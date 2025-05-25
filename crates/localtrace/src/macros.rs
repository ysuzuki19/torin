// SPDX-License-Identifier: MPL-2.0
#[macro_export]
macro_rules! trace {
    ($msg:expr) => {
        Err($crate::Error::new($msg))
    };

    ($fmt:expr, $($arg:tt)*) => {
        Err($crate::Error::new(format!($fmt, $($arg)*)))
    };
}
