use gdnative::api::InputEventMouseButton;
use gdnative::prelude::*;
use super::inspectors::*;
#[allow(clippy::unused_variable)]
#[derive(NativeClass)]
#[inherit(Control)]
pub struct DisallowClickFocus { }

#[methods]
impl DisallowClickFocus {
	fn new(_owner: &Control) -> Self {
		DisallowClickFocus { }
	}

	#[method]
	fn _gui_input(&self, #[base] _owner: &Control, event: Ref<InputEvent>) {
		unsafe {
			event.assume_safe().cast::<InputEventMouseButton>().touch_if_some(|event| {
				if event.is_pressed() {
					_owner.release_focus();
				}
			});
		}
	}
}