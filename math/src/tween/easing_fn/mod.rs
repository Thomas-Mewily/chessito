//! This package contains a set of simple easing functions. That consume a standardised `time`
//! attribute in the range between `0.0` and `1.0`, that represent the progress of a transition.
//! `0.0` being the beginning, `1.0` the end.
//!
//! They return a value between `0.0` and `1.0` (it might exceed the `0..=1` range temporarily
//! for a bounce effect). The returned value can be used to interpolate between the initial
//! (`0.0`) and the final (`1.0`) transition state, allowing for a "more natural" feel of
//! a transition by accelerating and decelerating at certain points, depending on the easing
//! function used.
//!
//! Visit [easings.net](https://easings.net/) to see visualisations of the different
//! easing functions.
//!
//! All easing functions have the same signature (`(f32) -> f32`) and can be easily stored as
//! fn pointers.
//!
//! ```
//! use ::simple_easing::linear;
//! let easing: fn(f32) -> f32 = linear;
//! assert_eq!(easing(1.0), 1.0);
//! ```
//! Some function are from https://crates.io/crates/simple-easing

mod back;
mod bounce;
mod circ;
mod cubic;
mod elastic;
mod expo;
mod quad;
mod quart;
mod quint;
mod sine;
mod mewily;

pub use back::*;
pub use bounce::*;
pub use circ::*;
pub use cubic::*;
pub use elastic::*;
pub use expo::*;
pub use quad::*;
pub use quart::*;
pub use quint::*;
pub use sine::*;
pub use mewily::*;

use crate::*;

pub fn linear(t: real) -> real {
	t
}

/// A linear easing that goes from `1.0` to `0.0`.
pub fn reverse(t: real) -> real {
	1.0 - t
}

/// A linear easing that goes from `0.0` to `1.0` and back to `0.0`. That might be used in
/// combination with other easing functions.
///
/// ## Example
/// ```
/// use ::simple_easing::{cubic_in, roundtrip};
/// let ascending = cubic_in(roundtrip(0.25));
/// let descending = cubic_in(roundtrip(0.75));
/// assert!((ascending - descending).abs() < 0.001);
/// ```
pub fn roundtrip(t: real) -> real {
	if t < 0.5 {
		t * 2.0
	} else {
		(1.0 - t) * 2.0
	}
}
