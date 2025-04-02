pub mod zero_sized_number;
pub use zero_sized_number::*;

#[macro_use]
pub mod map_on;
//pub use map_on::*;

pub mod integer;
pub use integer::*;

/// Module related to real (currently f32)
pub mod floating;
pub use floating::*;

/// The general interface for 2D, 3D and 4D coordinates
pub mod coordinate;
pub use coordinate::*;

/// 2D coordinates
pub mod coordinate2;
pub use coordinate2::*;

/// 3D coordinates
pub mod coordinate3;
pub use coordinate3::*;

/// 4D coordinates
pub mod coordinate4;
pub use coordinate4::*;

/// General interface for number
pub mod number;
pub use number::*;

/// A `N` dimension rectangle
pub mod rectangle;
pub use rectangle::*;

/// 2D Angle
pub mod angle;
pub use angle::*;

/// Vector of `bool` type with 2 dimensions
pub type Bool2 = C2<bool>;
pub const fn bool2(x: bool, y: bool) -> Bool2 { Bool2::new(x, y) }

/// Vector of `bool` type with 3 dimensions
pub type Bool3 = C3<bool>;
pub const fn bool3(x: bool, y: bool, z: bool) -> Bool3 { Bool3::new(x, y, z) }

/// Vector of `bool` type with 4 dimensions
pub type Bool4 = C4<bool>;
pub const fn bool4(x: bool, y: bool, z: bool, w: bool) -> Bool4 { Bool4::new(x, y, z, w) }


/// Vector of `real` type with 2 dimensions
pub type Vec2 = C2<real>;
pub const fn vec2(x: real, y: real) -> Vec2 { Vec2::new(x, y) }
pub const fn splat2<T : Copy>(xy : T) -> C2<T> { C2::<T>::splat_const(xy) }
pub const VEC2_ZERO : Vec2 = Vec2::ZERO; 
pub const VEC2_ONE  : Vec2 = Vec2::ONE; 
pub type Vec2Coef = Vec2;



/// Vector of `real` type with 3 dimensions
pub type Vec3 = C3<real>;
pub const fn vec3(x: real, y: real, z: real) -> Vec3 { Vec3::new(x, y, z) }
pub const fn splat3<T : Copy>(xyz : T) -> C3<T> { C3::<T>::splat_const(xyz) }
pub const VEC3_ZERO : Vec3 = Vec3::ZERO; 
pub const VEC3_ONE  : Vec3 = Vec3::ONE; 
pub type Vec3Coef = Vec3;



/// Vector of `real` type with 4 dimensions
pub type Vec4 = C4<real>;
pub const fn vec4(x: real, y: real, z: real, w:real) -> Vec4 { Vec4::new(x, y, z, w) }
pub const fn splat4<T : Copy>(xyzw : T) -> C4<T> { C4::<T>::splat_const(xyzw) }
pub const VEC4_ZERO : Vec4 = Vec4::ZERO; 
pub const VEC4_ONE  : Vec4 = Vec4::ONE; 
pub type Vec4Coef = Vec4;



/// Unsigned 2D point
pub type UPoint2 = C2<uint>;
pub fn upoint2(x: uint, y: uint) -> UPoint2 { UPoint2::new(x, y) }
pub const UPOINT2_ZERO : UPoint2 = UPoint2::ZERO; 
pub const UPOINT2_ONE  : UPoint2 = UPoint2::ONE; 

/// Unsigned 2D point
pub type UPoint3 = C3<uint>;
pub fn upoint3(x: uint, y: uint, z: uint) -> UPoint3 { UPoint3::new(x, y, z) }
pub const UPOINT3_ZERO : UPoint3 = UPoint3::ZERO; 
pub const UPOINT3_ONE  : UPoint3 = UPoint3::ONE; 

/// Unsigned 2D point
pub type UPoint4 = C4<uint>;
pub fn upoint4(x: uint, y: uint, z: uint, w: uint) -> UPoint4 { UPoint4::new(x, y, z, w) }
pub const UPOINT4_ZERO : UPoint4 = UPoint4::ZERO; 
pub const UPOINT4_ONE  : UPoint4 = UPoint4::ONE; 


/// Signed 2D point. Aka `IPoint2`. Allow for relative position, so it is preferable over `UPoint2` even if you got half the range
pub type Point2 = C2<int>;
pub fn point2(x: int, y: int) -> Point2 { Point2::new(x, y) }
pub const POINT2_ZERO : Point2 = Point2::ZERO; 
pub const POINT2_ONE  : Point2 = Point2::ONE; 

/// Signed 2D point. Aka `IPoint3`. Allow for relative position, so it is preferable over `UPoint3` even if you got half the range
pub type Point3 = C3<int>;
pub fn point3(x: int, y: int, z: int) -> Point3 { Point3::new(x, y, z) }
pub const POINT3_ZERO : Point3 = Point3::ZERO; 
pub const POINT3_ONE  : Point3 = Point3::ONE; 

/// Signed 2D point. Aka `IPoint4`. Allow for relative position, so it is preferable over `UPoint3` even if you got half the range
pub type Point4 = C4<int>;
pub fn point4(x: int, y: int, z: int, w: int) -> Point4 { Point4::new(x, y, z, w) }
pub const POINT4_ZERO : Point4 = Point4::ZERO; 
pub const POINT4_ONE  : Point4 = Point4::ONE; 



/// Unsigned 2D integer point. Aka `Point2UInt`
pub type Rect2U = Rectangle<UPoint2>;
/// Unsigned 3D integer point. Aka `Point3UInt`
pub type Rect3U = Rectangle<UPoint3>;
/// Unsigned 4D integer point. Aka `Point4UInt`
pub type Rect4U = Rectangle<UPoint4>;



/// Signed 2D integer point. Aka `Point2Int`
pub type Rect2I = Rectangle<Point2>;
/// Signed 3D integer point. Aka `Point3Int`
pub type Rect3I = Rectangle<Point3>;
/// Signed 4D integer point. Aka `Point4Int`
pub type Rect4I = Rectangle<Point4>;



/// 2D real point. Aka `Point2Real`
pub type Rect2 = Rectangle<Vec2>;
pub fn rect2(x: real, y: real, w: real, h: real) -> Rect2 { Rect2::new(vec2(x, y), vec2(w, h)) }
/// 3D real point. Aka `Point3Real`
pub type Rect3 = Rectangle<Vec3>;
/// 4D real point. Aka `Point4Real`
pub type Rect4 = Rectangle<Vec4>;

pub type Rect2Coef = Rect2;
pub type Rect3Coef = Rect3;
pub type Rect4Coef = Rect4;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() 
    {
        assert_eq!(upoint2(2, 4).y_x(), upoint2(4, 2));
        assert_eq!(upoint2(2, 4).x_x(), upoint2(2, 2));
        assert_eq!(upoint2(2, 4).y_y(), upoint2(4, 4));
        assert_eq!(point2(2, 4).rx_0(), point2(-2, 0));

        assert_eq!(point2(2, 4).to_vec2().length(), vec2(2.,4.).length());
    }
}
