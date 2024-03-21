use gdnative::api::*;
use gdnative::api::control::MouseFilter;
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use util::{fn_name, full_fn_name};
use crate::prelude::UnwrapManual;
use super::inspectors::*;

#[extends(SpinBox)]
pub struct DisallowSpinBoxLineEdit {}

#[methods]
impl DisallowSpinBoxLineEdit {
	#[method]
	fn _ready(&self, #[base] owner: &SpinBox) {
		owner.get_line_edit()
			 .touch_assert_sane(|line_edit| {
				 line_edit.set_mouse_filter(MouseFilter::IGNORE.into());
			 });
	}
}