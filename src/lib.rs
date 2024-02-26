#[macro_export]
macro_rules! ice {
    // If the invocation includes a label...
    ($label:ident => $expr:expr) => {{
        // ...define a named function whose definition will never be inlined...
        #[cold]
        #[inline(never)]
        fn $label<T, F: FnOnce() -> T>(f: F) -> T {
            f()
        }
        // ...and then wrap our `expr` in a closure and pass it to the function.
        $label(|| $expr)
    }};
    // If no label is specified, just define a closure to do the same thing.
    ($expr:expr) => {{
        // Closures that are passed in argument position are allowed to have annotations for
        // obscure historical reasons.
        let mut closure = core::convert::identity(
            #[cold]
            #[inline(never)]
            || $expr,
        );
        closure()
    }};
}
