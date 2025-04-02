use crate::*;

/// <https://easings.net/#easeInQuint>
pub fn quint_in(t: Coef) -> Coef {
	t * t * t * t
}

/// <https://easings.net/#easeOutQuint>
pub fn quint_out(t: Coef) -> Coef {
	1.0 - (1.0 - t).powi(5)
}

/// <https://easings.net/#easeInOutQuint>
pub fn quint_in_out(t: Coef) -> Coef {
	if t < 0.5 {
		16.0 * t * t * t * t * t
	} else {
		1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
	}
}
