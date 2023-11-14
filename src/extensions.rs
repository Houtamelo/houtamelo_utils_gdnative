use gdnative::prelude::*;
use gdnative::api::*;

pub trait TweenExtension {
    fn kill_if_valid(&self);
}

impl TweenExtension for Ref<SceneTreeTween> {
    fn kill_if_valid(&self) {
        let tween = unsafe { self.assume_safe() };
        if tween.is_valid() {
            tween.kill();
        }
    }
}

impl TweenExtension for &Ref<SceneTreeTween> {
    fn kill_if_valid(&self) {
        let tween = unsafe { self.assume_safe() };
        if tween.is_valid() {
            tween.kill();
        }
    }
}

pub trait OptionTweenExtension {
    fn kill_if_some(&mut self);
}

impl OptionTweenExtension for Option<Ref<SceneTreeTween>> {
    fn kill_if_some(&mut self) {
        if let Some(tween) = self {
            let tween = unsafe { tween.assume_safe() };
            if tween.is_valid() {
                tween.kill();
            }
        }

        *self = None;
    }
}

impl OptionTweenExtension for Option<&Ref<SceneTreeTween>> {
    fn kill_if_some(&mut self) {
        if let Some(tween) = self {
            let tween = unsafe { tween.assume_safe() };
            if tween.is_valid() {
                tween.kill();
            }
        }

        *self = None;
    }
}