use super::*;

pub mod tween;
pub use tween::*;

pub mod easing;
pub use easing::*;

/// Thank to https://docs.rs/simple-easing/latest/simple_easing/ for the easing function
/// I needed to edit it because I work with `real` and not `f32` 
pub mod easing_fn;
