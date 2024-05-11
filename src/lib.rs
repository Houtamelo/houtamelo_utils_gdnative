#![feature(let_chains)]
#![allow(unused_imports)]
#![feature(auto_traits)]
#![feature(negative_impls)]

mod disallow_click_focus;
mod inspectors;
mod tref_acquirer;
mod tref_unwrapper;
mod godot_log_macros;
mod extensions;
mod to_shared_array;
mod audio;
mod auto_text_resize;
mod disallow_spin_box_line_edit;
mod good_cell;
mod inherits;
mod connect_deferred;
mod loading;

pub mod prelude {
	pub use crate::inspectors::{manual, refcounted, instance, option_impls,
	                            GodotInstanceSomeInspector, GodotManualSomeInspector, GodotRefCountedSomeInspector, 
	                            SomeInspector, SomeMutInspector, ErrInspector, NoneInspector, OkInspector, OkMutInspector};
	pub use crate::disallow_click_focus::*;
	pub use crate::disallow_spin_box_line_edit::*;
	pub use crate::godot_error_get;
	pub use crate::godot_panic;
	pub use crate::tref_acquirer::*;
	pub use crate::tref_unwrapper::*;
	pub use crate::extensions::*;
	pub use crate::to_shared_array::*;
	pub use crate::audio::*;
	pub use crate::auto_text_resize::*;
	pub use crate::good_cell::*;
	pub use crate::inherits::*;
	pub use crate::connect_deferred::*;
	pub use crate::loading::*;
	pub use gdnative::api::scene_tree_tween::TweenPauseMode;
	pub use gdnative::api::tween::TweenProcessMode;
	pub use gdnative::object::memory::Memory;
	pub use gdnative::derive::*;
	pub use gdnative::prelude::*;
	pub use gdnative::api::*;
	pub use gdnative_export_node_as_path::extends;
}
