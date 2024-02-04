#![allow(unused_imports)]

use gdnative::init::{godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init };
use gdnative::prelude::*;

mod disallow_click_focus;
mod inspectors;
mod tref_acquirer;
mod tref_unwrapper;
mod godot_log_macros;
mod extensions;
mod to_shared_array;
mod audio;

fn init(handle: InitHandle) {
	handle.add_class::<prelude::DisallowClickFocus>();
	handle.add_class::<audio::PitchRandomizer>();
	handle.add_class::<audio::PlayOnHoverAndPitchRandomizer>();
	handle.add_class::<audio::PlayOnClickAndPitchRandomizer>();
}

godot_gdnative_init!(_ as houtamelo_utils_gdnative_init);
godot_nativescript_init!(init as houtamelo_utils_gdnative_nativescript_init);
godot_gdnative_terminate!(_ as houtamelo_utils_gdnative_terminate);

pub mod prelude {
	pub use crate::inspectors::*;
	pub use crate::inspectors::option_impls::*;
	pub use crate::inspectors::manual::*;
	pub use crate::inspectors::refcounted::*;
	pub use crate::inspectors::instance::*;
	pub use crate::disallow_click_focus::*;
	pub use crate::godot_error_get;
	pub use crate::godot_panic;
	pub use crate::tref_acquirer::*;
	pub use crate::tref_unwrapper::*;
	pub use crate::extensions::*;
	pub use crate::to_shared_array::*;
}
