use crate::*;

const C1: real = 1.70158;
const C2: real = C1 * 1.525;
const C3: real = C1 + 1.0;

/// <https://easings.net/#easeInBack>
pub fn back_in(t: Coef) -> Coef {
	C3 * t * t * t - C1 * t * t
}

/// <https://easings.net/#easeOutBack>
pub fn back_out(t: Coef) -> Coef {
	1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// <https://easings.net/#easeInOutBack>
pub fn back_in_out(t: Coef) -> Coef {
	if t < 0.5 {
		((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
	} else {
		((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
	}
}
