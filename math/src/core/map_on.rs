pub use crate::zero_sized_number::*;


/// `i8`, `i16`, `i32`, `i64`, `isize`
#[macro_export]
macro_rules! map_on_non_zero_signed {
    ($mac:ident) => {
        $mac!(i8);
        $mac!(i16);
        $mac!(i32);
        $mac!(i64);
        $mac!(isize);
    };
}

/// `i0`
#[macro_export]
macro_rules! map_on_zero_signed { ($mac:ident) => { $mac!($crate::zero_sized_number::i0); };}

/// `i0` + `i8`, `i16`, `i32`, `i64`, `isize`
#[macro_export]
macro_rules! map_on_signed {
    ($mac:ident) => {
        $crate::map_on_zero_signed!($mac);
        $crate::map_on_non_zero_signed!($mac);
    };
}



/// `u8`, `u16`, `u32`, `u64`, `usize`
#[macro_export]
macro_rules! map_on_non_zero_unsigned {
    ($mac:ident) => {
        $mac!(u8);
        $mac!(u16);
        $mac!(u32);
        $mac!(u64);
        $mac!(usize);
    };
}

/// `u0`
#[macro_export]
macro_rules! map_on_zero_unsigned { ($mac:ident) => { $mac!($crate::zero_sized_number::u0); };}

/// `u0`, `u8`, `u16`, `u32`, `u64`, `usize`
#[macro_export]
macro_rules! map_on_unsigned {
    ($mac:ident) => {
        $crate::map_on_zero_unsigned!($mac);
        $crate::map_on_non_zero_unsigned!($mac);
    };
}




/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`)
#[macro_export]
macro_rules! map_on_non_zero_integer {
    ($mac:ident) => {
        $crate::map_on_non_zero_unsigned!($mac);
        $crate::map_on_non_zero_signed!($mac);
    };
}

/// (`i0`, `u0`)
#[macro_export]
macro_rules! map_on_zero_integer {
    ($mac:ident) => {
        $mac!(i0);
        $mac!(u0);
    };
}

/// (`i0`, `i8`, `i16`, `i32`, `i64`, `isize`) + (`u0`, `u8`, `u16`, `u32`, `u64`, `usize`)
#[macro_export]
macro_rules! map_on_integer {
    ($mac:ident) => {
        $crate::map_on_unsigned!($mac);
        $crate::map_on_signed!($mac);
    };
}




/// `f32`, `f64`
#[macro_export]
macro_rules! map_on_non_zero_floating {
    ($mac:ident) => {
        $mac!(f32);
        $mac!(f64);
    };
}

/// `f0`
#[macro_export]
macro_rules! map_on_zero_floating { ($mac:ident) => { $mac!($crate::zero_sized_number::f0); };}

/// `f0`, `f32`, `f64`
#[macro_export]
macro_rules! map_on_floating {
    ($mac:ident) => {
        $crate::map_on_zero_floating!($mac);
        $crate::map_on_non_zero_floating!($mac);
    };
}


/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`)
#[macro_export]
macro_rules! map_on_non_zero_scalar {
    ($mac:ident) => 
    {
        $crate::map_on_non_zero_integer!($mac);
        $crate::map_on_non_zero_floating!($mac);
    };
}

/// (`i0`, `u0`, `f0`)
#[macro_export]
macro_rules! map_on_zero_scalar {
    ($mac:ident) => 
    {
        $crate::map_on_zero_integer!($mac);
        $crate::map_on_zero_floating!($mac);
    };
}


/// (`i0`, `i8`, `i16`, `i32`, `i64`, `isize`) + (`u0`, `u8`, `u16`, `u32`, `u64`, `usize`) + (`f0`, `f32`, `f64`)
#[macro_export]
macro_rules! map_on_scalar {
    ($mac:ident) => 
    {
        $crate::map_on_integer!($mac);
        $crate::map_on_floating!($mac);
    };
}
