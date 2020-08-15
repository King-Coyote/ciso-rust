#[macro_export]
macro_rules! safe_context {
    ($e:expr, $c:expr) => {
        $e.lock().unwrap().lua.context($c);
    };
}

#[macro_export]
macro_rules! widget_table {
    ($ctx:expr, $id:expr) => {
        $ctx.globals()
            .get::<_, Table>("Gui")?
            .get::<_, Table>("widgets")?
            .get::<_, Table>($id)?;
    };
}