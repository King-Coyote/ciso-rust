#[macro_export]
macro_rules! safe_context {
    ($e:expr, $c:expr) => {
        $e.lock().unwrap().lua.context($c);
    };
}