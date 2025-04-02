use crate::*;

/// <https://easings.net/#easeInCubic>
pub fn cubic_in(t: Coef) -> Coef {
	t * t * t
}

/// <https://easings.net/#easeOutCubic>
pub fn cubic_out(t: Coef) -> Coef {
	1.0 - (1.0 - t).powi(3)
}

/// <https://easings.net/#easeInOutCubic>
pub fn cubic_in_out(t: Coef) -> Coef {
	if t < 0.5 {
		4.0 * t * t * t
	} else {
		1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
	}
}
