use crate::prelude::*;
use util::prelude::*;

#[extends(Label)]
pub struct AutoTextResize {
	#[property] min_size: i64,
	#[property] max_size: i64,
	owned_font: Option<Ref<DynamicFont>>,
}

#[methods]
impl AutoTextResize {
	#[method]
	fn _ready(&mut self, #[base] owner: &Label) {
		if self.max_size == 0 {
			godot_warn!(
				"{fn_name}: max_size is not set to a positive value. Defaulting to current font_size.\n\
				 Object: {node_name}", 
				fn_name = fn_name(&AutoTextResize::_ready), node_name = owner.name());
			self.max_size = 
				owner.get_font("font", "")
					 .and_then(|font| font.cast::<DynamicFont>())
					 .map(|font| unsafe { font.assume_safe().size() })
					 .unwrap_or(16);
		}
		
		if self.min_size == 0 {
			godot_warn!(
				"{fn_name}: min_size is not set to a positive value. Defaulting to 6.\n\
				 Object: {node_name}", 
				fn_name = fn_name(&AutoTextResize::_ready), node_name = owner.name());
			self.min_size = 6;
		}
		
		if owner.max_lines_visible() <= 0 {
			owner.set_max_lines_visible(1);
			godot_warn!(
				"{fn_name}: max_lines_visible is not set to a positive value. Defaulting to 1.\n\
				 Object: {node_name}", 
				fn_name = fn_name(&AutoTextResize::_ready), node_name = owner.name());
		}

		if !owner.has_autowrap() {
			owner.set_autowrap(true);
			godot_warn!(
				"{fn_name}: autowrap is not set to true. Overriding to true.\n\
				 Object: {node_name}",
				fn_name = fn_name(&AutoTextResize::_ready), node_name = owner.name());
		}

		self.update_font_size(owner);
	}

	fn set_min_size(&mut self, owner: &Label, value: Variant) {
		if let Ok(size) = value.try_to::<i64>() {
			self.min_size = size;
			self.update_font_size(owner);
		} else {
			godot_warn!(
				"{fn_name}: Failed to convert value to i64.\n\
				 Value: {value}\n\
				 Object: {node_name}", 
				fn_name = fn_name(&AutoTextResize::set_min_size), value = value.to_string(), node_name = owner.name());
		}
	}

	fn set_max_size(&mut self, owner: &Label, value: Variant) {
		if let Ok(size) = value.try_to::<i64>() {
			self.max_size = size;
			self.update_font_size(owner);
		} else {
			godot_warn!(
				"{fn_name}: Failed to convert value to i64.\n\
				 Value: {value}\n\
				 Object: {node_name}", 
				fn_name = fn_name(&AutoTextResize::set_max_size), value = value.to_string(), node_name = owner.name());
		}
	}

	fn set_text(&mut self, owner: &Label, value: Variant) {
		if let Ok(text) = value.try_to::<GodotString>() {
			owner.set_text(text);
			owner.set_clip_text(true);
			self.update_font_size(owner);
		} else {
			godot_warn!(
				"{fn_name}: Failed to convert value to GodotString.\n\
				 Value: {value}\n\
				 Object: {node_name}",
				fn_name = fn_name(&AutoTextResize::set_text), value = value.to_string(), node_name = owner.name());
		}
	}

	#[method]
	fn _set(&mut self, #[base] owner: &Label, property: String, value: Variant) -> bool {
		return match property.as_str() {
			"min_size" => {
				self.set_min_size(owner, value);
				true
			},
			"max_size" => {
				self.set_max_size(owner, value);
				true
			},
			"text" => {
				self.set_text(owner, value);
				true
			},
			_ => false,
		};
	}

	fn get_or_create_owned_font(&mut self, owner: &Label) -> Option<Ref<DynamicFont>> {
		if let Some(already_owned) = &self.owned_font {
			return Some(already_owned.clone());
		}

		let original_ref =
			owner.get_font("font", "")
			     .or_else(|| owner.get_theme_default_font())
			     .and_then(|font_ref| font_ref.cast::<DynamicFont>())?;

		let original = unsafe { original_ref.assume_safe() };

		let owned_font_ref =
			original.duplicate(false)
			        .and_then(|resource| resource.cast::<DynamicFont>())?;

		let owned_font = unsafe { owned_font_ref.assume_safe() };
		owned_font.set_size(self.max_size);
		self.owned_font = Some(owned_font_ref.clone());
		owner.add_font_override("font", owned_font_ref.clone());

		return Some(owned_font_ref);
	}

	#[method]
	fn update_font_size(&mut self, #[base] owner: &Label) {
		let Some(font_ref) = self.get_or_create_owned_font(owner)
			else {
				godot_warn!(
					"{}: Failed to get or create owned font.\n\
					 Object: {}", fn_name(&AutoTextResize::update_font_size), owner.name());
				return;
			};

		let font =
			unsafe { font_ref.assume_safe() };

		font.set_size(self.max_size);
		font.update_changes();

		let mut font_size = self.max_size;

		while owner.get_visible_line_count() < owner.get_line_count()
			&& font_size > self.min_size {
			font_size -= 1;
			font.set_size(font_size);
			font.update_changes();
		}
	}
}