use crate::*;

/// <https://easings.net/#easeInExpo>
pub fn expo_in(t: Coef) -> Coef {
	if t <= 0.0 {
		0.0
	} else {
		2.to_real().powf(10.0 * t - 10.0)
	}
}

/// <https://easings.net/#easeOutExpo>
pub fn expo_out(t: Coef) -> Coef {
	if 1.0 <= t {
		1.0
	} else {
		1.0 - 2.to_real().powf(-10.0 * t)
	}
}

/// <https://easings.net/#easeInOutExpo>
pub fn expo_in_out(t: Coef) -> Coef {
	if t <= 0.0 {
		0.0
	} else if 1.0 <= t {
		1.0
	} else if t < 0.5 {
		2.to_real().powf(20.0 * t - 10.0) / 2.0
	} else {
		(2.0 - 2.to_real().powf(-20.0 * t + 10.0)) / 2.0
	}
}
