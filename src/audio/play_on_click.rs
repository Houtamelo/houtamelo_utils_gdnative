use crate::prelude::*;
use util::prelude::*;

use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[extends(AudioStreamPlayer2D)]
#[derive(Debug)]
pub struct PlayOnClickAndPitchRandomizer {
	original_pitch: f64,
}

#[methods]
impl PlayOnClickAndPitchRandomizer {
	#[method]
	fn _ready(&mut self, #[base] owner: &AudioStreamPlayer2D) {
		self.original_pitch = owner.pitch_scale();

		let owner_ref = unsafe { owner.assume_shared() };
		let parent_option = owner.get_parent();
		let parent = parent_option.unwrap_manual();

		if parent.has_signal("pressed") {
			parent.connect("pressed", owner_ref, fn_name(&Self::_play_custom),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
		} else if parent.has_signal("gui_input") {
			parent.connect("gui_input", owner_ref, fn_name(&Self::_on_gui_input),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
		} else if parent.has_signal("input_event") {
			parent.connect("input_event", owner_ref, fn_name(&Self::_on_input_event),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
		} else {
			godot_warn!("{}():\n\
			 Node `{}` cannot connect to it's parent `{}`\n\
			 Parent does not have any of these signals: `gui_input` | `pressed` | `input_event`",
				full_fn_name(&Self::_ready), owner.name(), parent.name());
		}
	}

	#[method]
	fn _on_gui_input(&self, #[base] owner: &AudioStreamPlayer2D, event: Ref<InputEvent>) {
		if is_confirm_input(event) {
			self._play_custom(owner);
		}
	}

	#[method]
	fn _on_input_event(&self, #[base] owner: &AudioStreamPlayer2D, _viewport: Ref<Node>, event: Ref<InputEvent>, _shape_idx: i64) {
		if is_confirm_input(event) {
			self._play_custom(owner);
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

fn is_confirm_input(event: Ref<InputEvent>) -> bool {
	let safe_event = unsafe { event.assume_safe() };

	if safe_event.cast::<InputEventMouseButton>().is_some_and(|mouse_event|
		mouse_event.is_pressed() && mouse_event.button_index() == GlobalConstants::BUTTON_LEFT)
	{
		return true;
	}

	if safe_event.is_action_pressed("ui_accept", false, true) {
		return true;
	}

	return false;
}