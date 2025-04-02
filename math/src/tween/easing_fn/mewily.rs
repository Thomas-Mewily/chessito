use crate::*;

pub fn pow_in(t : Coef, n : Coef) -> Coef { t.powf(n) }
pub fn pow_out(t : Coef, n : Coef) -> Coef { 1.-(1.-t).powf(n) }


pub fn smooth(t : Coef, n : Coef) -> Coef { if t < 0.5 {2.to_real().powf(n-1.)* t.powf(n)} else { 1. -(((-2.*t+2.).powf(n))/2.) }}

/// round
pub fn retro(t : Coef, n : Coef) -> Coef { round(t, n) }
pub fn round(t : Coef, n : Coef) -> Coef { (t * n) as int as Coef / n }
