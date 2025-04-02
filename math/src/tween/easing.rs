use crate::*;
use std::{
    default,
    fmt::{Debug, Formatter, Result},
    ops::*,
};

/// Thank to https://easings.net/# for some common easing function, and the nice visualisation
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum EasingFn {
    #[default]
    Linear,
    Reverse,
    Roundtrip,

    /// x^n
    PowIn(real),

    /// Generalized https://easings.net/#easeOutCubic for every power
    ///
    /// 1-(1-x)^n
    PowOut(real),

    /// Smooth in and out
    ///
    /// Generalized https://easings.net/#easeInOutCubic for every power
    ///
    /// if x < 0.5 {2^(n-1)*x^n} else { 1-(((-2*x+2)^n)/(2))) }
    Smooth(real),

    /// Nb step
    Round(real),

    /// https://easings.net/#easeInBack
    BackIn,
    /// https://easings.net/#easeOutBack
    BackOut,
    /// https://easings.net/#easeInOutBack
    BackInOut,

    /// https://easings.net/#easeInBounce
    BounceIn,
    /// https://easings.net/#easeOutBounce
    BounceOut,
    /// https://easings.net/#easeInOutBounce
    BounceInOut,

    /// https://easings.net/#easeInCirc
    CircIn,
    /// https://easings.net/#easeOutCirc
    CircOut,
    /// https://easings.net/#easeInOutCirc
    CircInOut,

    /// https://easings.net/#easeInCubic
    CubicIn,
    /// https://easings.net/#easeOutCubic
    CubicOut,
    /// https://easings.net/#easeInOutCubic
    CubicInOut,

    /// https://easings.net/#easeInElastic
    ElasticIn,
    /// https://easings.net/#easeOutElastic
    ElasticOut,
    /// https://easings.net/#easeInOutElastic
    ElasticInOut,

    /// https://easings.net/#easeInExpo
    ExpoIn,
    /// https://easings.net/#easeOutExpo
    ExpoOut,
    /// https://easings.net/#easeInOutExpo
    ExpoInOut,

    /// https://easings.net/#easeInQuad
    QuadIn,
    /// https://easings.net/#easeOutQuad
    QuadOut,
    /// https://easings.net/#easeInOutQuad
    QuadInOut,

    /// https://easings.net/#easeInQuart
    QuartIn,
    /// https://easings.net/#easeOutQuart
    QuartOut,
    /// https://easings.net/#easeInOutQuart
    QuartInOut,

    /// https://easings.net/#easeInSine
    SineIn,
    /// https://easings.net/#easeOutSine
    SineOut,
    /// https://easings.net/#easeInOutSine
    SineInOut,
}

impl EasingFn {
    /// Limited between [0, 1]
    pub fn apply(&self, c: Coef) -> Coef {
        if c <= 0. {
            return 0.;
        }
        if c >= 1. {
            return 1.;
        }
        return self.apply_not_limited(c);
    }

    pub fn apply_not_limited(self, t: Coef) -> Coef {
        use easing_fn::*;
        match self {
            EasingFn::Linear => linear(t),
            EasingFn::Reverse => reverse(t),
            EasingFn::Roundtrip => roundtrip(t),
            EasingFn::PowIn(n) => pow_in(t, n),
            EasingFn::PowOut(n) => pow_out(t, n),
            EasingFn::Smooth(n) => smooth(t, n),
            EasingFn::Round(n) => round(t, n),

            EasingFn::BackIn => back_in(t),
            EasingFn::BackOut => back_out(t),
            EasingFn::BackInOut => back_in_out(t),

            EasingFn::BounceIn => bounce_in(t),
            EasingFn::BounceOut => bounce_out(t),
            EasingFn::BounceInOut => bounce_in_out(t),

            EasingFn::CircIn => circ_in(t),
            EasingFn::CircOut => circ_out(t),
            EasingFn::CircInOut => circ_in_out(t),

            EasingFn::CubicIn => cubic_in(t),
            EasingFn::CubicOut => cubic_out(t),
            EasingFn::CubicInOut => cubic_in_out(t),

            EasingFn::ElasticIn => elastic_in(t),
            EasingFn::ElasticOut => elastic_out(t),
            EasingFn::ElasticInOut => elastic_in_out(t),

            EasingFn::ExpoIn => expo_in(t),
            EasingFn::ExpoOut => expo_out(t),
            EasingFn::ExpoInOut => expo_in_out(t),

            EasingFn::QuadIn => quad_in(t),
            EasingFn::QuadOut => quad_out(t),
            EasingFn::QuadInOut => quad_in_out(t),

            EasingFn::QuartIn => quart_in(t),
            EasingFn::QuartOut => quart_out(t),
            EasingFn::QuartInOut => quart_in_out(t),

            EasingFn::SineIn => sine_in(t),
            EasingFn::SineOut => sine_out(t),
            EasingFn::SineInOut => sine_in_out(t),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Easing {
    kind: EasingFn,
}

impl Easing {
    const fn new(kind: EasingFn) -> Self {
        Self { kind }
    }

    /// Restricted between [0, 1]
    pub fn apply(self, c: Coef) -> Coef { self.kind.apply(c) }
    /// Not restricted between [0, 1]
    pub fn apply_unrestricted(self, c: Coef) -> Coef { self.kind.apply_not_limited(c) }

    pub const fn linear() -> Self { Self::new(EasingFn::Linear) }
    pub const fn reverse() -> Self { Self::new(EasingFn::Reverse) }
    pub const fn roundtrip() -> Self { Self::new(EasingFn::Roundtrip) }

    pub const fn robot() -> Self { Self::new(EasingFn::ElasticInOut) }

    pub const fn retro(n : real) -> Self { Self::new(EasingFn::Round(n)) }

    /// https://easings.net/#easeInQuad
    ///
    /// pow_in(2.)
    pub const fn pow_quad_in() -> Self { Self::pow_in(2.) }
    /// https://easings.net/#easeInCubic
    ///
    /// pow_in(3.)
    pub const fn pow_cubic_in() -> Self { Self::pow_in(3.) }
    /// https://easings.net/#easeInQuart
    ///
    /// pow_in(4.)
    pub const fn pow_quart_in() -> Self { Self::pow_in(4.) }
    pub const fn pow_in(n: real) -> Self { Self::new(EasingFn::PowIn(n)) }

    /// https://easings.net/#easeOutQuad
    ///
    /// pow_out(2.)
    pub const fn pow_quad_out() -> Self { Self::pow_out(2.) }
    /// https://easings.net/#easeOutCubic
    ///
    /// pow_out(3.)
    pub const fn pow_cubic_out() -> Self { Self::pow_out(3.) }
    /// https://easings.net/#easeOutQuart
    ///
    /// pow_out(4.)
    pub const fn pow_quart_out() -> Self { Self::pow_out(4.) }
    pub const fn pow_out(n: real) -> Self { Self::new(EasingFn::PowOut(n)) }

    /// https://easings.net/#easeInBack
    pub const fn back_in() -> Self { Self::new(EasingFn::BackIn) }
    /// https://easings.net/#easeOutBack
    pub const fn back_out() -> Self { Self::new(EasingFn::BackOut) }
    /// https://easings.net/#easeInOutBack
    pub const fn back_in_out() -> Self { Self::new(EasingFn::BackInOut) }

    /// https://easings.net/#easeInBounce
    pub const fn bounce_in() -> Self { Self::new(EasingFn::BounceIn) }
    /// https://easings.net/#easeOutBounce
    pub const fn bounce_out() -> Self { Self::new(EasingFn::BounceOut) }
    /// https://easings.net/#easeInOutBounce
    pub const fn bounce_in_out() -> Self { Self::new(EasingFn::BounceInOut) }

    /// https://easings.net/#easeInCirc
    pub const fn circ_in() -> Self { Self::new(EasingFn::CircIn) }
    /// https://easings.net/#easeOutCirc
    pub const fn circ_out() -> Self { Self::new(EasingFn::CircOut) }
    /// https://easings.net/#easeInOutCirc
    pub const fn circ_in_out() -> Self { Self::new(EasingFn::CircInOut) }

    /// https://easings.net/#easeInCubic
    pub const fn cubic_in() -> Self { Self::new(EasingFn::CubicIn) }
    /// https://easings.net/#easeOutCubic
    pub const fn cubic_out() -> Self { Self::new(EasingFn::CubicOut) }
    /// https://easings.net/#easeInOutCubic
    pub const fn cubic_in_out() -> Self { Self::new(EasingFn::CubicInOut) }

    /// https://easings.net/#easeInElastic
    pub const fn elastic_in() -> Self { Self::new(EasingFn::ElasticIn) }
    /// https://easings.net/#easeOutElastic
    pub const fn elastic_out() -> Self { Self::new(EasingFn::ElasticOut) }
    /// https://easings.net/#easeInOutElastic
    pub const fn elastic_in_out() -> Self { Self::new(EasingFn::ElasticInOut) }

    /// https://easings.net/#easeInExpo
    pub const fn expo_in() -> Self { Self::new(EasingFn::ExpoIn) }
    /// https://easings.net/#easeOutExpo
    pub const fn expo_out() -> Self { Self::new(EasingFn::ExpoOut) }
    /// https://easings.net/#easeInOutExpo
    pub const fn expo_in_out() -> Self { Self::new(EasingFn::ExpoInOut) }

    /// https://easings.net/#easeInQuad
    pub const fn quad_in() -> Self { Self::new(EasingFn::QuadIn) }
    /// https://easings.net/#easeOutQuad
    pub const fn quad_out() -> Self { Self::new(EasingFn::QuadOut) }
    /// https://easings.net/#easeInOutQuad
    pub const fn quad_in_out() -> Self { Self::new(EasingFn::QuadInOut) }

    /// https://easings.net/#easeInQuart
    pub const fn quart_in() -> Self { Self::new(EasingFn::QuartIn) }
    /// https://easings.net/#easeOutQuart
    pub const fn quart_out() -> Self { Self::new(EasingFn::QuartOut) }
    /// https://easings.net/#easeInOutQuart
    pub const fn quart_in_out() -> Self { Self::new(EasingFn::QuartInOut) }

    /// https://easings.net/#easeInSine
    pub const fn sine_in() -> Self { Self::new(EasingFn::SineIn) }
    /// https://easings.net/#easeOutSine
    pub const fn sine_out() -> Self { Self::new(EasingFn::SineOut) }
    /// https://easings.net/#easeInOutSine
    pub const fn sine_in_out() -> Self { Self::new(EasingFn::SineInOut) }
}
