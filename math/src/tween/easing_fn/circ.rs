use crate::*;

/// <https://easings.net/#easeInCirc>
pub fn circ_in(t: Coef) -> Coef {
	1.0 - (1.0 - t.powi(2)).sqrt()
}

/// <https://easings.net/#easeOutCirc>
pub fn circ_out(t: Coef) -> Coef {
	(1.0 - (t - 1.0).powi(2)).sqrt()
}

/// <https://easings.net/#easeInOutCirc>
pub fn circ_in_out(t: Coef) -> Coef {
	if t < 0.5 {
		(1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
	} else {
		((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
	}
}
