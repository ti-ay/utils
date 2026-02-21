//! Holds the macro [`time_it`].


/// Prints the time it took to execute a code block with a variable metric from seconds to microseconds.
/// Default is disabled.
/// 
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_time = ["time_it/enable_timing"]
/// dont_print_time = ["time_it/disable_timing"]
/// ```
///
/// ### Examples
/// ```ignore
/// use time_it::time_it;
/// let _ = time_it!(fib(40));
/// // [fib(40)] took: 570 ms
/// let _ = time_it!("Fibonacci of 40", fib(40));
/// // [Fibonacci of 40] took: 570 ms
/// ```
#[macro_export]
macro_rules! time_it {
    ($label:expr ,$code:expr) => {{ $crate::__internal_variable_print_time($label, || $code) }};

    ($code:expr) => {
        $crate::time_it!(stringify!($code), $code)
    };
}

#[inline(always)]
pub fn __internal_variable_print_time<F, R>(_label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    #[cfg(all(feature = "enable_timing", feature = "disable_timing"))]
    compile_error!(
        "Cannot use both 'enable_timing' and 'disable_timing' in crate 'time_it'"
    );

    #[cfg(all(feature = "enable_timing", not(feature = "disable_timing")))]
    {
        let start = std::time::Instant::now();
        let result = f();
        crate::variable_print_time(_label, start.elapsed());
        result
    }

    #[cfg(any(not(feature = "enable_timing"), feature = "disable_timing"))]
    {
        f()
    }
}

#[cfg(all(feature = "enable_timing", not(feature = "disable_timing")))]
fn variable_print_time(label: &str, duration: std::time::Duration) {
    eprint!("[{}] took: ", label);
    if duration > std::time::Duration::from_secs(10) {
        eprintln!("{} s", duration.as_secs());
        return;
    }

    if duration < std::time::Duration::from_millis(1) {
        eprintln!("{} μs", duration.as_micros());
        return;
    }
    eprintln!("{} ms", duration.as_millis());
}
