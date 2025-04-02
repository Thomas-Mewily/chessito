use crate::*;

/// <https://easings.net/#easeInBounce>
pub fn bounce_in(t: Coef) -> Coef {
	1.0 - bounce_out(1.0 - t)
}

/// <https://easings.net/#easeOutBounce>
pub fn bounce_out(t: Coef) -> Coef {
	const N1: real = 7.5625;
	const D1: real = 2.75;
	if t < 1.0 / D1 {
		return N1 * t * t;
	} else if t < 2.0 / D1 {
		return N1 * (t - 1.5 / D1).powi(2) + 0.75;
	} else if t < 2.5 / D1 {
		return N1 * (t - 2.25 / D1).powi(2) + 0.9375;
	} else {
		return N1 * (t - 2.625 / D1).powi(2) + 0.984375;
	}
}

/// <https://easings.net/#easeInOutBounce>
pub fn bounce_in_out(t: Coef) -> Coef {
	if t < 0.5 {
		(1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
	} else {
		(1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
	}
}
