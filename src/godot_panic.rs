#[macro_export]
macro_rules! godot_panic {
    ($msg:expr) => {
        {
            godot_error!("{}", $msg);
            panic!("{}", $msg);
        }
    };
}