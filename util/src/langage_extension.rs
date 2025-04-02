// I did not use cargo because the code is not long, and I try to avoid dependency
// Code by https://github.com/dpc/rust-default


/// A shorthand for `Default::default()` that you can `use default::default`.
///
/// See `https://internals.rust-lang.org/t/could-we-have-std-default/8756` for
/// discussion
pub fn ___<T:Default>()-> T {
    std::default::Default::default() // gosh, that's a lot of default, isn't it?
}

pub trait DefaultExtension
{
    fn ___() -> Self;
}
impl<T:Default> DefaultExtension for T
{
    fn ___() -> Self {

        Self::default()
    }
}

// Thank to @ctrl-alt-delor on `https://stackoverflow.com/questions/67872308/how-to-check-if-for-loop-is-on-the-last-element-of-an-iterator`
pub trait IterEndPeek 
{ 
    fn is_last(&mut self) -> bool;
    fn is_not_last(&mut self) -> bool { !self.is_last() }
}
impl<I: Iterator> IterEndPeek for  std::iter::Peekable<I> 
{
    fn is_last(&mut self) -> bool { self.peek().is_none() }
}


pub trait IterIsEmpty { fn is_empty(&mut self) -> bool; }
impl<I: Iterator> IterIsEmpty for I
{
    fn is_empty(&mut self) -> bool { self.next().is_none() }
}

#[macro_export]
macro_rules! vec_into  
{ 
    // Base case for empty input
    () => {
        Vec::new()
    };

    // Recursive case for non-empty input
    ($val:expr $(, $tokens:expr)*) => {{
        let mut vec = Vec::with_capacity(0 $(+ {let _ = stringify!($tokens); 1})*);
        vec.push($val.into());
        $( vec.push($tokens.into()); )*
        vec
    }};

    // With extra comma
    ($val:expr $(, $tokens:expr)* ,) => {{
        let mut vec = Vec::with_capacity(0 $(+ {let _ = stringify!($tokens); 1})*);
        vec.push($val.into());
        $( vec.push($tokens.into()); )*
        vec
    }};
}



/// made by Mewily :3
pub trait KeywordTrait
{
    fn with<F>(&mut self, f : F) -> &mut Self where F : Fn(&mut Self) { f(self); self}
    fn is<F>(&self, f : F) -> bool where F : Fn(&Self) -> bool { f(self) }
}
impl<T> KeywordTrait for T {}