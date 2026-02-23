//! Holds the internal method [`crate::out::__internal_print_out`].

/// Prints a message without a newline.
/// After the message stdout is flushed.
/// Default is disabled.
/// 
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_out = ["ut/out_on"]
/// dont_print_out = ["ut/out_off"]
/// ```
///
/// ### Examples
/// ```ignore
/// use ut::ut_out;
/// ut_out!("Hello world!");
/// ut_out!("Hello world!");
/// // Hello world!Hello world!
/// ```
#[macro_export]
macro_rules! ut_out {
    ($($args:tt)*) => {
        $crate::out::__internal_print_out(format_args!($($args)*), false);
    };
}


/// Prints a message with a newline.
/// After the message stdout is flushed.
/// Default is disabled.
/// 
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_out = ["ut/out_on"]
/// dont_print_out = ["ut/out_off"]
/// ```
///
/// ### Examples
/// ```ignore
/// use ut::ut_outln;
/// ut_outln!("Hello world!");
/// ut_outln!("Hello world!");
/// // Hello world!
/// // Hello world!
/// ```
#[macro_export]
macro_rules! ut_outln {
    ($($args:tt)*) => {
        $crate::out::__internal_print_out(format_args!($($args)*), true);
    };
}

#[doc(hidden)]
pub fn __internal_print_out(_args: std::fmt::Arguments, _newline: bool) {
    #[cfg(all(feature = "out_on", feature = "out_off"))]
    {
        compile_error!(
            "Cannot use both 'out_on' and 'out_off' for macro 'ut_out'"
        );
    }
    
    #[cfg(all(feature = "out_on", not(feature = "out_off")))]
    {
        use std::io::Write;
        let mut stdout = std::io::stdout();
        if _newline {
            let _ = writeln!(stdout, "{}", _args);
        } else {
            let _ = write!(stdout, "{}", _args);
        }
        let _ = stdout.flush();
    }
}