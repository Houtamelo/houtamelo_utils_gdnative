use crate::prelude::*;
use util::prelude::*;

use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[extends(AudioStreamPlayer2D)]
#[derive(Debug)]
pub struct PlayOnHoverAndPitchRandomizer {
	original_pitch: f64,
}

#[methods]
impl PlayOnHoverAndPitchRandomizer {
	#[method]
	fn _ready(&mut self, #[base] owner: &AudioStreamPlayer2D) {
		self.original_pitch = owner.pitch_scale();

		let owner_ref = unsafe { owner.assume_shared() };
		let parent_option = owner.get_parent();
		let parent = parent_option.unwrap_manual();
		
		if parent.has_signal("mouse_entered") {
			parent.connect("mouse_entered", owner_ref, fn_name(&PlayOnHoverAndPitchRandomizer::_play_custom),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			      .log_if_err();
		} else {
			godot_warn!("{}():\n\
			Node `{}` cannot connect to it's parent `{}`\n\
			Parent does not have signal `mouse_entered`.",
				full_fn_name(&Self::_ready), owner.name(), parent.name());
		}
		
		if parent.has_signal("focus_entered") {
			parent.connect("focus_entered", owner_ref, fn_name(&PlayOnHoverAndPitchRandomizer::_play_custom),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			      .log_if_err();
		}
	}

	#[method]
	fn _play_custom(&self, #[base] owner: &AudioStreamPlayer2D) {
		let mut rng = Xoshiro256PlusPlus::from_entropy();
		let pitch = self.original_pitch * (0.9 + rng.gen_range(0.0..=0.2));
		owner.set_pitch_scale(pitch);
		owner.play(0.0);
	}
}