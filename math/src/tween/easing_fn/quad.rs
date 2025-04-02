use crate::*;

/// <https://easings.net/#easeInQuad>
pub fn quad_in(t: Coef) -> Coef {
	t * t
}

/// <https://easings.net/#easeOutQuad>
pub fn quad_out(t: Coef) -> Coef {
	1.0 - (1.0 - t).powi(2)
}

/// <https://easings.net/#easeInOutQuad>
pub fn quad_in_out(t: Coef) -> Coef {
	if t < 0.5 {
		2.0 * t * t
	} else {
		1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
	}
}
