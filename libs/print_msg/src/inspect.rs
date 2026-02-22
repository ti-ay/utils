//! Holds the macro [`inspect`].

/// Prints the value with a description or the code itself.
/// Default is disabled.
/// 
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_inspect = ["print_msg/enable_inspect"]
/// dont_print_inspect = ["print_msg/disable_inspect"]
/// ```
///
/// ### Examples
/// ```ignore
/// use print_msg::inspect;
/// inspect!("This is a test", 5);
/// // [INSPECT] "This is a test" => 5
/// inspect!(5);
/// // [INSPECT] "5" => 5
/// ```
#[macro_export]
macro_rules! inspect {
    ($desc:expr, $val: expr) => {
        $crate::inspect::__internal_print_inspect($desc, $val);
    };

    ($val: expr) => {
        $crate::inspect!(stringify!($val), $val);
    };
}

#[inline(always)]
pub fn __internal_print_inspect<T>(_desc: &str, _value: T)
where
    T: std::fmt::Debug,
{
    #[cfg(all(feature = "enable_inspect", feature = "disable_inspect"))]
    {
        compile_error!(
            "Cannot use both 'enable_inspect' and 'disable_inspect' for macro 'inspect'"
        );
    }
    #[cfg(all(feature = "enable_inspect", not(feature = "disable_inspect")))]
    {
        println!("[INSPECT] \"{}\" => {:?}", _desc, _value);
    }
}
