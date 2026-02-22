//! Holds the internal method [`crate::view::__internal_print_view`].

/// Prints the value with a description or the code itself.
/// Default is disabled.
/// 
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_view = ["ut/view_on"]
/// dont_print_view = ["ut/view_off"]
/// ```
///
/// ### Examples
/// ```ignore
/// use ut::view;
/// view!("This is a test", 5);
/// // [VIEW] "This is a test" => 5
/// view!(5);
/// // [VIEW] "5" => 5
/// ```
#[macro_export]
macro_rules! view {
    ($desc:expr, $val: expr) => {
        $crate::view::__internal_print_view($desc, $val);
    };

    ($val: expr) => {
        $crate::view!(stringify!($val), $val);
    };
}

#[inline(always)]
pub fn __internal_print_view<T>(_desc: &str, _value: T)
where
    T: std::fmt::Debug,
{
    #[cfg(all(feature = "view_on", feature = "view_off"))]
    {
        compile_error!(
            "Cannot use both 'view_on' and 'view_off' for macro 'view'"
        );
    }
    #[cfg(all(feature = "view_on", not(feature = "view_off")))]
    {
        println!("[VIEW] \"{}\" => {:?}", _desc, _value);
    }
}
