use crate::*;

/// <https://easings.net/#easeInSine>
pub fn sine_in(t: Coef) -> Coef {
	1.0 - (t * real::PI / 2.0).cos()
}

/// <https://easings.net/#easeOutSine>
pub fn sine_out(t: Coef) -> Coef {
	(t * real::PI / 2.0).sin()
}

/// <https://easings.net/#easeInOutSine>
pub fn sine_in_out(t: Coef) -> Coef {
	-((real::PI * t).cos() - 1.0) / 2.0
}
