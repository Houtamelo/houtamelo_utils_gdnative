use anyhow::anyhow;
use crate::prelude::*;
use util::prelude::*;
use crate::prelude::control::MouseFilter;
use crate::prelude::input::CursorShape;

#[extends(SpinBox)]
pub struct DisallowSpinBoxLineEdit {}

#[methods]
impl DisallowSpinBoxLineEdit {
	#[method]
	fn _ready(&self, #[base] owner: &SpinBox) {
		owner.get_line_edit()
		     .ok_or_else(|| anyhow!("SpinBox::get_line_edit() returned `None`."))
		     .and_then(|l_ref| unsafe {
			     let line_edit =
				     l_ref.assume_safe_if_sane()
				          .ok_or_else(|| anyhow!("SpinBox::get_line_edit() returned a null reference."))?;

			     line_edit.connect("focus_entered", owner.assume_shared(), fn_name(&Self::_on_line_edit_focus_entered),
			                       l_ref.to_shared_array(), Object::CONNECT_DEFERRED)
			              .log_if_err();

			     line_edit.set_default_cursor_shape(CursorShape::ARROW.into());
			     Ok(())
		     }).log_if_err();
	}

	#[method]
	fn _on_line_edit_focus_entered(&self, line_edit: Ref<LineEdit>) {
		line_edit.touch_assert_sane(|line_edit| {
			line_edit.release_focus();
		});
	}
}