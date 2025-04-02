use crate::*;

const C4: real = (2.0 * real::PI) / 3.0;
const C5: real = (2.0 * real::PI) / 4.5;

/// <https://easings.net/#easeInElastic>
pub fn elastic_in(t: Coef) -> Coef {
	if t <= 0.0 {
		0.0
	} else if 1.0 <= t {
		1.0
	} else {
		-2.to_real().powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
	}
}

/// <https://easings.net/#easeOutElastic>
pub fn elastic_out(t: Coef) -> Coef {
	if t <= 0.0 {
		0.0
	} else if 1.0 <= t {
		1.0
	} else {
		2.to_real().powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
	}
}

/// <https://easings.net/#easeInOutElastic>
pub fn elastic_in_out(t: Coef) -> Coef {
	if t <= 0.0 {
		0.0
	} else if 1.0 <= t {
		1.0
	} else if t < 0.5 {
		-(2.to_real().powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
	} else {
		(2.to_real().powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
	}
}
