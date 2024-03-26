#![feature(let_chains)]
#![allow(unused_imports)]

use gdnative::prelude::*;

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

pub mod prelude {
	pub use crate::inspectors::{manual, refcounted, instance, option_impls,
	                            GodotInstanceSomeInspector, GodotManualSomeInspector, GodotRefCountedSomeInspector, 
	                            SomeInspector, SomeMutInspector, ErrInspector, NoneInspector, OkInspector, OkMutInspector};
	pub use crate::disallow_click_focus::DisallowClickFocusOnParent;
	pub use crate::disallow_spin_box_line_edit::DisallowSpinBoxLineEdit;
	pub use crate::godot_error_get;
	pub use crate::godot_panic;
	pub use crate::tref_acquirer::{AssertSafeTRef, AssertSaneTRef, AssertTInstance, TrySafeTRef, TrySaneTRef};
	pub use crate::tref_unwrapper::{UnwrapInstance, UnwrapManual, UnwrapRefCount};
	pub use crate::extensions::{TweenExtension, OptionTweenExtension};
	pub use crate::to_shared_array::IntoSharedArray;
	pub use crate::audio::{PlayOnHoverAndPitchRandomizer, PlayOnClickAndPitchRandomizer, PitchRandomizer};
	pub use crate::auto_text_resize::AutoTextResize;
	pub use crate::good_cell::*;
}
