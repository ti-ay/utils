//! Holds the internal method [`crate::time::__internal_variable_print_time`].


/// Prints the time it took to execute a code block with a variable metric from seconds to microseconds.
/// Default is disabled.
/// 
/// ### Setup
/// ```toml
/// [features]
/// default = []
/// print_time = ["ut/time_on"]
/// dont_print_time = ["ut/time_off"]
/// ```
///
/// ### Examples
/// ```ignore
/// use ut::ut_time;
/// let _ = ut_time!(fib(40));
/// // [TIME] [fib(40)] took: 570 ms
/// let _ = ut_time!("Fibonacci of 40", fib(40));
/// // [TIME] [Fibonacci of 40] took: 570 ms
/// ```
#[macro_export]
macro_rules! ut_time {
    ($label:expr ,$code:expr) => {{ $crate::time::__internal_variable_print_time($label, || $code) }};

    ($code:expr) => {
        $crate::ut_time!(stringify!($code), $code)
    };
}

#[inline(always)]
pub fn __internal_variable_print_time<F, R>(_label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    #[cfg(all(feature = "time_on", feature = "time_off"))]
    compile_error!(
        "Cannot use both 'time_on' and 'time_off' for macro 'ut_time'"
    );

    #[cfg(all(feature = "time_on", not(feature = "time_off")))]
    {
        let start = std::time::Instant::now();
        let result = f();
        crate::time::variable_print_time(_label, start.elapsed());
        result
    }

    #[cfg(any(not(feature = "time_on"), feature = "time_off"))]
    {
        f()
    }
}

#[cfg(all(feature = "time_on", not(feature = "time_off")))]
fn variable_print_time(label: &str, duration: std::time::Duration) {
    eprint!("[TIME] [{}] took: ", label);
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
