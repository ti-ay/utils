//! Holds the internal method [`crate::note::__internal_print_note`].

/// Prints a message with a prefix.
/// Default is disabled.
///
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_note = ["ut/note_on"]
/// dont_print_note = ["ut/note_off"]
/// ```
/// ### Examples
/// ```ignore
/// use ut::note;
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
    #[cfg(all(feature = "note_on", feature = "note_off"))]
    compile_error!("Cannot use both 'note_on' and 'note_off' for the macro 'note'");

    #[cfg(all(feature = "note_on", not(feature = "note_off")))]
    {
        std::eprintln!("[NOTE] {}", _args);
    }
}
