use crate::*;

/// <https://easings.net/#easeInQuart>
pub fn quart_in(t: Coef) -> Coef {
	t * t * t * t
}

/// <https://easings.net/#easeOutQuart>
pub fn quart_out(t: Coef) -> Coef {
	1.0 - (1.0 - t).powi(4)
}

/// <https://easings.net/#easeInOutQuart>
pub fn quart_in_out(t: Coef) -> Coef {
	if t < 0.5 {
		8.0 * t * t * t * t
	} else {
		1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
	}
}
