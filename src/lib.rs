use gdnative::init::{ godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init };
use gdnative::prelude::*;

pub(crate) mod bounded_ints;

pub mod clamp01;
pub mod hashset_macro;
pub mod disallow_click_focus;
pub mod inspectors;
pub mod tref_acquirer;
pub mod tref_unwrapper;
pub mod godot_log_macros;

fn init(handle: InitHandle) {
	handle.add_class::<prelude::DisallowClickFocus>();
}

godot_gdnative_init!(_ as houtamelo_utils_gdnative_init);
godot_nativescript_init!(init as houtamelo_utils_gdnative_nativescript_init);
godot_gdnative_terminate!(_ as houtamelo_utils_gdnative_terminate);

pub mod prelude {
	pub use crate::bounded_ints::*;
	pub use crate::inspectors::*;
	pub use crate::inspectors::option_impls::*;
	pub use crate::inspectors::manual::*;
	pub use crate::inspectors::refcounted::*;
	pub use crate::inspectors::instance::*;
	pub use crate::clamp01::*;
	pub use crate::disallow_click_focus::*;
	pub use crate::godot_error_get;
	pub use crate::godot_panic;
	pub use crate::hash_set;
	pub use crate::tref_acquirer::*;
	pub use crate::tref_unwrapper::*;
}
