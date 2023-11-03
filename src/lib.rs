use gdnative::init::{godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init};
use gdnative::prelude::*;

pub mod bounded_isize;
pub mod bounded_u32;
pub mod clamp01;
pub mod hashset_macro;
pub mod disallow_click_focus;
pub mod inspectors;
pub mod tref_acquirer;

pub use crate::bounded_isize::*;
pub use crate::bounded_u32::*;
pub use crate::clamp01::*;
pub use crate::disallow_click_focus::*;
pub use crate::inspectors::*;
pub use crate::tref_acquirer::*;

fn init(handle: InitHandle) {
	handle.add_class::<DisallowClickFocus>();
}

godot_gdnative_init!(_ as houtamelo_utils_gdnative_init);
godot_nativescript_init!(init as houtamelo_utils_gdnative_nativescript_init);
godot_gdnative_terminate!(_ as houtamelo_utils_gdnative_terminate);