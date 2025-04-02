# Programming Convention / Guideline

## A flexible, fast Rust Game Engine ⚡

## Before you read 📖

This document is a set of rule used in the code to make it clearer.

This document **don't** dictate how to write code, but try to collect a list a good pratices **from** the code already written to use when coding, and can be changed/updated if needed.

This document should be used as a Tool Box 🧰, editing and changing tools is always an option.

Some listing showcased in this document may be not updated.
En cas de doute, check the documentation (`cargo doc` command) or the code source itself for more details, and update this document. 🔧

## Fonctions

### Functions and Methods :

`Methods` are recognizable because they take `self` by value or by reference as the first argument using the `self`, `&self` or `&mut self` sugar syntax. `Methods` can be called on a object using the `subject.verb(complement)` syntax.
Under the hood, a `method` is just some regular `function` with some sugar syntax to use this syntax. 

`Function` on the other hand can't be called using the `subject.verb(complement)`, but `subjectType::verb(complement)` : They don't take a self by value in argument.
- `u32_to_char(u32) -> Option<char>`
- `a_function(v : Self) -> i32` not a method, because can't be call using the `value.a_function()` syntax. You must use `ValueType::a_function(value)` instead.

Sometime the term `Function` will be used to designate a `Method` by abuse of language (even too it is correct, but less precise.)

### `With` function

Function starting with `with_X` are *call by value* : They Consume `self` in the argument, and return `self`

```rust
fn with_X(self, ...) -> Self {...}
```

Calling those functions (methods more specifically) can be chained ⛓️ :
```rust
let foo = vec4(1., 2., 3., 4.);
let bar = foo.with_x(42.).with_z(32.); // produce a new vec4 : (42., 2., 32., 4.)
```

### `Set` function

Setter functions (or methods more specifically) start with `set_X` and are *call by reference* : 
They take `&mut self` in the argument, and return the same `self` passed in argument (of type `&mut Self`)

```rust
fn set_X(&mut self, ...) -> &mut Self { ... self }
```

Returning  `&mut Self` allow to chain ⛓️ calls like with `with_X` functions :

```rust
let mut foo = vec4(1., 2., 3., 4.);
foo.set_x(42.).set_z(32.); // edit foo to be (42., 2., 32., 4.)
```

Other helper function like `add`, `move` are also called by (mutable) reference

```rust
let mut foo = vec4(1., 2., 3., 4.);
foo.move_x(42.).move_z(32.); // (43., 2., 34., 4.)
```

### Function returning a `Reference`

Simple getter function name don't start with `get_X` and are just named `X` : 

- `fn x(&self) -> &X` 👍
- `fn get_x(&self) -> &X` 👎

ex :
```rust
fn last(&self) -> &T { &self.last }
```

They generally return a reference, but in some specific case, if the object returned have the Copy trait, it is ok to return a copy of the value rather than a reference :

```rust
let x : i32 = vec2(1, 2).x(); // return a copy of the X axis
```

Function starting by `get` are used for more complexe function like indexing (ex: in a grid) : `fn get(index : Point2) -> &T`. Thoses structs also define the `Index` trait. (same for `get_mut` and `IndexMut`)

#### `mutable` `Reference` :

Function that return a simple reference with little to no code (sometime they can return a reference into an `Option` or a `Result` if the operation can fail) can also define their mutable equivalent if needed (returning a mutable reference) :

Those function end with `_mut` :
```rust
fn last_mut(&mut self) -> &mut T { &mut self.last }
```

### Function returning Boolean ❓

Simple function returning boolean generally start with `is_X`, `have_X`. ❓

```rust
let b : bool  = foo.have_length();
```

More complexe function are free to have whatever name is effective to describe them : 

ex : the `any` function :
ex `vec![0,1,2,3,4].any(|nb| nb % 2 == 0)` will check if the vector contains any even number.
The `any` function is part of the Second-order logic, the `any` function take a closure/function in parameter.

## Primitive 💻

### Rust Type 👨‍💻

Rust define integer of various size named `iX` where `X` is either : `8`,`16`,`32`,`64`... or `size` (for native size : ex: `isize`). (Respectivly `i8`, `i16`, `i32`, `i64`, `isize`) (It is also possible to use greater than 64 bits integer).

Same for unsigned, they are name `uX`, and the type `usize` can be used for indexing.

Floating point also exist with different precision : 32 and 64 bits : `f32` and `f64`.


Having some abstract definition for `signed integer`, `unsigned integer` and `floating` independent to the precision is important for a abstract math library.


### Math Type 

The type `real` is the main wrapper for a floating value.
Currenlty is a typedef to `f32` (because the game engine is currently 32 bits), but it can be changed later for a `f64` without having to change any code !

Same for the type `int`, it is the main wrapper for an integer type. (currently `isize`, same size as the machine native size).

And lastly, `uint` is the main unsigned type.
It's representation is `usize` (same size as the machine native size).
This type should not be used to index vector, because the typedef can be changed.


I can't prevent programmer using it as an index/`usize`, because wrapping the `uint` inside a custom struct will change the way to create an unsigned integer :
- `let v : uint = 42` 👍 
- `let v : uint = 42.to_uint()` 👎 (and I don't like this syntax for creating such a simple number)


There is also the type `Coef`, and it will always be a typedef for a `real`, but it exist to highlight that the value stored inside should be used as a coefficient (`Coef` can be less than 0. or greater than 1.)

## Common Interface

To be able to use integer and floating numbers in an interesting way in generic type, some trait must be define to declare which operation are available on them. (like defining addition, multiplication, comparison...)

The main idea for those traits is to differenciate `Unit` (unit of measurement) and `Number` (the primitive value `uX`, `iX`, `fX`).

### Unit of Measurement 📏

Trait with `Unit` inside define operation for unit of measurement. The operation must to coherent from the dimensional analysis perspective.

- `2🍎 + 3🍎 = 5🍎` ✔️
- `2🍎 * 3🍎 = ❓💥❗` ❌


For instance, you can add 2 distances, and it should give you a distance. But you can't multiply 2 distances, because the result is not a distance (rather an distance^2 which should be a different type).



Ex: The `Time` 🕒 struct is used to define some time related value (with an `real` precision). You can instanciate it with `2000.ms()`, `10.5.s()` or even `2.mins()`. When reading the value (of type) `real`, you need to specify which unit do you want :

```rust
let t            : Time = 1500.ms();
let second       : real = t.s(); // 1.5
let whole_second : uint = t.whole_s(); // 1
```

#### Unit Sheet

##### Time :
🕒 `TimeClock<T : Unit + DefaultDeltaTime>` : Define a generic time unit 

🕒 `Time = TimeClock<real>` : a time used for storing contiguous time (real) 
🕒 `Tick = TimeClock<int>` : a time used for incremental/turn based time (int)

##### Math :
📐`Angle` : support the unit : `degree`, `radian` and `turn`. Also support trigonometric function. ex `10.degree().cos()`

##### User Interface ⚙️

⚙️ `UiUnit` : Define a length in pixel independently of the screen resolution 🖥️ using multiples coefficients depending on multiple factor (the windows size, the container parent size...)



##### Other
Other example of not coded yet unit of mesurement :
- 🌡️ Temperature
- 📏 Distance

### Number / Primitive 🔢

Trait with `Number` inside define operation for any kind of number. They are more abstract than unit of measurement, and don't carry any dimension information.

Integer and floating point are `Number` for instance. Multiplying 2 integers together is fine, and it should give you back an integer.

The math crate also define `u0`, `i0` and `f0`. Those are `ZST` (Zero Sized Type) and implement the same interface as their non-zero equivalent.

The macro `map_on_X` 🛎️ allow to easily apply a rust macro to a group of a given type

##### Signed
`map_on_non_zero_signed` : `i8`, `i16`, `i32`, `i64`, `isize`
`map_on_zero_signed` : `i0`
`map_on_signed` : `i0` + `i8`, `i16`, `i32`, `i64`, `isize`

##### Unsigned
`map_on_non_zero_unsigned` : `u8`, `u16`, `u32`, `u64`, `usize`
`map_on_zero_unsigned` : `u0` 
`map_on_unsigned` : `u0`, `u8`, `u16`, `u32`, `u64`, `usize`

##### Integer
`map_on_non_zero_integer` : (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`)
`map_on_zero_integer` : (`i0`, `u0`)
`map_on_integer` : (`i0`, `i8`, `i16`, `i32`, `i64`, `isize`) + (`u0`, `u8`, `u16`, `u32`, `u64`, `usize`)

##### Floating
`map_on_non_zero_floating` : `f32`, `f64`
`map_on_zero_floating` : `f0`
`map_on_zero_floating` : `f0`, `f32`, `f64`

##### Scalar
`map_on_non_zero_scalar` : (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`)
`map_on_zero_scalar` : (`i0`, `u0`, `f0`)
`map_on_scalar` : (`i0`, `i8`, `i16`, `i32`, `i64`, `isize`) + (`u0`, `u8`, `u16`, `u32`, `u64`, `usize`) + (`f0`, `f32`, `f64`)


For example, here is how the `HaveZero` trait is implemented for the primtives :

```rust
/// Define the `0` representation
pub trait HaveZero 
{ 
    const ZERO : Self;
    fn is_zero(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::ZERO }
    fn is_non_zero(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_zero() }
}

// For Scalar
macro_rules! impl_have_zero {
    ($primitive_name: ty) => 
    { impl HaveZero for $primitive_name { const ZERO : Self = 0 as Self; } };
}
map_on_non_zero_scalar!(impl_have_zero);

// For ZST
macro_rules! impl_have_zero_for_zst {
    ($primitive_name: ty) => 
    { impl HaveZero for $primitive_name { const ZERO : Self = Self; } };
}
map_on_zero_scalar!(impl_have_zero_for_zst);

// Disputable
impl HaveZero for bool { const ZERO : Self = false; }
```


### Coordinate 🗺️

For any 2D, 3D, and 4D coordinate of any kind, there is a generic `C2<T>`, `C3<T>` and `C4<T>` struct that exist. 🗺️

```rust
type Bool2   = C2<bool>
type Vec2    = C2<real>;
type UPoint2 = C2<uint>;
type Point2  = C2<int>;
type UiVec   = C2<UiUnit>;

/// Just for clarity, will always be a Vec2
pub type Vec2Coef = Vec2;
```

Replace `2` with `3` or `4` to change the dimension.

There is also a generic `Rectangle<T>` struct that define common function (ex : if a point is inside) for coordinate of any dimension (2D, 3D, 4D).

```rust
type UiRect = Rectangle<UiVec>;
type Rect2U = Rectangle<UPoint2>;
type Rect2I = Rectangle<Point2>;
type Rect2  = Rectangle<Vec2>;
```

### Traits 🔥

Here is a list of the trait related to math, number and coordinate.
A lot of thought and brainpower went into designing these traits. 🧠

###### Number Traits :

- ➕ `Signed` : number that can store signed value 
- ➖ `Unsigned` : number that can store unsigned value (ex int, uint, and also float). 

- `Absolute` : define the absolute function.

- `ToUint`
- `ToInt`
- `ToReal`, `FromReal`

- 0️⃣ `HaveZero` : The `ZERO` representation. 
- `HaveHalf` : The `HALF` representation. 
- 1️⃣ `HaveOne`  : The `ONE`  representation. 
- `HaveMinusOne` : The `MINUS_ONE` representation.

- `SmallestIncrement` : Define the smallest reasonable increment. `1` for integer, 1/60. for float (can change).

- `RealNumber : FloatingNumber + UnitArithmetic<real>` : only for the real number
- `FloatingNumber : Scalar + HaveHalf`


- `UnsignedIntegerOrFloatingNumber : Scalar` : useful for coordinate conversion
- `SignedIntegerOrFloatingNumber : Scalar` : useful for coordinate conversion

- `HaveMinMaxValue` : define `MIN_VAL` and `MAX_VAL`


- `SignedInteger : Eq + Ord + Scalar`
```rust
UnsignedInteger : Eq + Ord + Scalar + 
    Shl<Self, Output=Self> + ShlAssign<Self> +
    Shr<Self, Output=Self>  + ShlAssign<Self> +
    BitOr<Self, Output=Self> + BitOrAssign<Self> +
    BitAnd<Self, Output=Self> + BitAndAssign<Self> + 
    BitXor<Self, Output=Self> + BitXorAssign<Self> +
    Not<Output = Self>
```
Bit operation are only defined on `unsigned` `integer`

- `Integer : Eq + Ord + Scalar` : 

- `HaveDefaultRange : HaveMinMaxValue + ToReal + FromReal + UnitArithmetic<Self>` : For floating the range is : `[0., 1.]`. For integers the range is : `[0, Self::MAX_VAL]`


- ➕ ➖ 🟰 🍎
```rust
UnitArithmetic<T=Self> :
    Copy + Clone +
    Add<T,Output=Self> + Sub<T,Output=Self> + 
    AddAssign<T> + SubAssign<T> +
    HaveZero + HaveOne + Absolute + 
    PartialEq +
    Debug
```

- ✖️ ➗ 🔢
```rust
NumberArithmetic<T=Self> :
    UnitArithmetic<Self> + 
    Mul<T,Output=Self> + Div<T,Output=Self> + Rem<T,Output=Self> +
    MulAssign<T> + DivAssign<T> + RemAssign<T> +
```

- 🟰 `NumberCompare : PartialEq + PartialOrd + PartialOrdComparison` define `==`, `>=`, `<=` and `!=`
- 🟰 `PartialOrdComparison` is also implemented for float

- 🍎 `Unit : UnitArithmetic<Self> + NumberCompare + SmallestIncrement`
- 🔢 `Number : Unit + NumberArithmetic<Self>`


- `Scalar : Number + ToReal + FromReal + ToInt + ToUint + HaveMinMaxValue + HaveDefaultRange`
for integer + floating

- `Reversable` : For number : `-x`/`x*-1`, for boolean : `!x`

###### Coordinate Traits : ✨

- `CoordinateFloatingNumber : Sized + CoordinateNumber + NumberArithmetic<Self> ` : define `length()` and `normalize()` and related functions.

- `CoordinateInteger : Sized + CoordinateUnit` : define a way to iterate on the area `fn iter_area(self) -> impl Iterator<Item = Self>`

```rust
CoordinateNumber where Self : CoordinateUnit + NumberArithmetic, <Self as CoordinateUnit>::Precision : NumberArithmetic
```
Define function that can't exist on unit : `length_squared()`, `area()`, `min_element()` / `max_element()`...
*(Yes `min_element()` / `max_element()` are here, and a they name suggest, they return the smallest/biggest `X`/`Y`/`Z`/`W` component of the coordinate)*

- `CoordinateUnit where Self : UnitArithmetic<Self>` : basic interface for 2D up to 4D coordinate.

##### Some design choice

The reason why `min_element()` / `max_element()` are in `CoordinateNumber` and not `CoordinateUnit` is because you can't directly compare unit.

Some unit like distance can be compared, but not all of them.

If you want to compare a distance `a` and `b`, you need to compare `a.meter()` and `b.meter()`, and the meter is not a unit.
But for unit like Angle, how would you compare 2 angle to get the minimal angle ? It don't make sense, you have multiple way to compare them depending on what you want.


ex : A `UiUnit` is a composed of multiple coefficient depending on multiple factor (the windows size, the container parent size...). It represent a length in pixel, independently of the screen resolution. For different screen size, the same UiUnit can produce different pixel length.

`UiUnit` are Unit : they can be added and substracted together... but they can't be multiplied together.
They also can't be compared together, because to define if an UiUnit is bigger/smaller than another, we should compare the pixel length, but the pixel length depend of the screen resolution, and UiUnit are independent to screen resolution.

One way to make it more flexible is to introduce more traits like `CoordinateNumberArithmetic` and `CoordinateUnitArithmetic`

# Author
- Thomas Mewily 2024


draft :
other cool emoji : ✨🌀💧🔋💡🧲⭐