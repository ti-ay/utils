/// Prints a message with a prefix.
/// Default is disabled.
///
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_note = ["print_msg/enable_note"]
/// dont_print_note = ["print_msg/disable_note"]
/// ```
/// ### Examples
/// ```ignore
/// use print_msg::note;
/// note!("Hello world!");
/// // [NOTE] Hello world!
/// ```
#[macro_export]
macro_rules! note {
    ($($args:tt)*) => {
        $crate::note::__internal_print_note(format_args!($($args)*));
    };
}

#[inline(always)]
pub fn __internal_print_note(_args: std::fmt::Arguments) {
    #[cfg(all(feature = "enable_note", feature = "disable_note"))]
    compile_error!("Cannot use both 'enable_note' and 'disable_note' for the macro 'note'");

    #[cfg(all(feature = "enable_note", not(feature = "disable_note")))]
    {
        std::eprintln!("[NOTE] {}", _args);
    }
}
