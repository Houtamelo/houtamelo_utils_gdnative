use gdnative::api::*;
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;

#[extends(Label)]
pub struct AutoTextResize {
	#[property] min_size: i64,
	#[property] max_size: i64,
	owned_font: Option<Ref<DynamicFont>>,
}

#[methods]
impl AutoTextResize {
	#[method]
	fn _ready(&self, #[base] owner: &Label) {
		owner.set_clip_text(true);
	}

	#[method]
	fn _set(&mut self, #[base] owner: &Label, property: String, value: Variant) -> bool {
		if property != "text" {
			return false;
		}

		let Ok(text) = value.try_to::<GodotString>()
			else { 
				godot_warn!(
					"_set::text: Failed to convert value to GodotString.\n\
					 Value: {}\n\
					 Object: {}", value.to_string(), owner.name()
				);
				return false;
			};
		
		owner.set_text(text);
		owner.set_clip_text(true);
		self.update_font_size(owner);
		return true;
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
		
		return Some(owned_font_ref);
	}

	#[method]
	fn update_font_size(&mut self, #[base] owner: &Label) {
		let Some(font_ref) = self.get_or_create_owned_font(owner)
			else {
				godot_warn!(
					"{}: Failed to get or create owned font.\n\
					 Object: {}", util::fn_name(&AutoTextResize::update_font_size), owner.name());
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