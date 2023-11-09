#[macro_export]
macro_rules! godot_panic {
    ($msg:expr) => {
        {
            godot_error!("{}", $msg);
            panic!("{}", $msg)
        }
    }
}

#[macro_export]
macro_rules! godot_error_get {
    ($msg:expr) => {
        {
            godot_error!("{}", $msg);
            $msg
        }
    }
}