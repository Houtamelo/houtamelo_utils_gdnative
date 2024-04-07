use crate::prelude::*;
use util::*;
use crate::prelude::control::MouseFilter;

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