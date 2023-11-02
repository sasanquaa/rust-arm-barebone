#[macro_export]
macro_rules! debug_assert_call_once {
    () => {
        debug_assert_call_once!("assertion failed: function can only be called once");
    };
    ($($arg:tt)+) => {{
        if cfg!(debug_assertions) {
            fn assert_has_not_been_called() {
                use core::sync::atomic::{AtomicBool, Ordering};
                static CALLED: AtomicBool = AtomicBool::new(false);
                let called = CALLED.swap(true, Ordering::Relaxed);
                debug_assert!(called == false, $($arg)+);
            }
            assert_has_not_been_called();
        }
    }};
}

#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => {{
        use $crate::io::device::uart::KUart;
        KUart.print_str($fmt);
        KUart.print_str("\n");
    }};
    ($fmt:expr, $($args:tt)*) => {{
        todo!()
    }};
}
