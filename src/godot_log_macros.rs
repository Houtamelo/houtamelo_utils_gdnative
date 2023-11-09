#[macro_export]
macro_rules! godot_panic {
    ($($args:tt)*) => ({
        let msg = std::format!($($args)*);
        gdnative::log::error(gdnative::godot_site!(), msg.as_str());
        panic!("{msg}");
    });
}

#[macro_export]
macro_rules! godot_error_get {
    ($($args:tt)*) => ({
        #[allow(unused_variables)]
        let msg = std::format!($($args)*);
        #[allow(unused_variables)]
        gdnative::log::error(gdnative::godot_site!(), msg.as_str());
        msg
    });
}