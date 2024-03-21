use gdnative::api::*;
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use crate::prelude::UnwrapManual;
use super::inspectors::*;

#[extends(Node)]
pub struct DisallowClickFocusOnParent { }

#[methods]
impl DisallowClickFocusOnParent {
	#[method]
	fn _ready(&self, #[base] owner: &Node) {
		let owner_ref = unsafe { owner.assume_shared() };
		let parent_option = owner.get_parent();
		let parent = parent_option.unwrap_manual();

		if parent.has_signal("pressed") {
			parent.connect("pressed", owner_ref, util::fn_name(&Self::_on_pressed),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
		} else if parent.has_signal("gui_input") {
			parent.connect("gui_input", owner_ref, util::fn_name(&Self::_on_gui_input),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			      .log_if_err();
		} else if parent.has_signal("input_event") {
			parent.connect("input_event", owner_ref, util::fn_name(&Self::_on_input_event),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
		} else {
			godot_warn!("{}():\n\
			 Node `{}` cannot connect to it's parent `{}`\n\
			 Parent does not have any of these signals: `gui_input` | `pressed` | `input_event`",
				util::full_fn_name(&Self::_ready), owner.name(), parent.name());
		}
	}
	
	fn release_parent_focus(&self, owner: &Node) {
		let Some(parent) = (
			unsafe { 
				owner.get_parent()
					 .and_then(|parent| 
						 parent.assume_safe_if_sane()) 
			}) else { return };
		
		if parent.has_method("release_focus") {
			unsafe {
				parent.call_deferred("release_focus", &[]);
			}
		} else {
			godot_warn!("{}():\n\
			 Node `{}` cannot release focus from it's parent `{}`\n\
			 Parent does not have method `release_focus`",
				util::full_fn_name(&Self::_on_gui_input), owner.name(), parent.name());
		}
	}

	#[method]
	fn _on_gui_input(&self, #[base] owner: &Node, event: Ref<InputEvent>) {
		if is_mouse_pressed(event) {
			self.release_parent_focus(owner);
		}
	}

	#[method]
	fn _on_input_event(&self, #[base] owner: &Node, _viewport: Ref<Node>, event: Ref<InputEvent>, _shape_idx: i64) {
		if is_mouse_pressed(event) {
			self.release_parent_focus(owner);
		}
	}

	#[method]
	fn _on_pressed(&self, #[base] owner: &Node) {
		self.release_parent_focus(owner);
	}
}

fn is_mouse_pressed(event: Ref<InputEvent>) -> bool {
	return event.cast::<InputEventMouseButton>()
	            .is_some_and(|event| 
		            unsafe { event.assume_safe().is_pressed() });
}