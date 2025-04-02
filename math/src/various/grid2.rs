
use std::{clone, fmt::{Debug, Display, Formatter, Result}, marker::PhantomData, ops::{Index, IndexMut, Range}};
use crate::*;


#[derive(Default, PartialEq, Eq, Clone)]
pub struct Grid2<T>
{
    size  : Point2,
    tiles : Vec<T>,
}

impl<T> Grid2<T> {
    pub fn iter(&self) -> impl Iterator<Item=&T> { self.tiles.iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> { self.tiles.iter_mut() }
    pub fn iter_idx(&self) -> impl Iterator<Item=Point2> { self.size.iter_area() }
}

impl<T> Grid2<T>
{
    pub fn new_map<F>(mut size : Point2, f : F) -> Self where F : Fn(Point2) -> T 
    {
        size = size.max(Point2::ZERO);
        Self
        {
            size,
            tiles : 
            {
                let mut v = Vec::with_capacity(size.sum_axis().to_uint());
                for idx in 0.. size.x * size.y
                {
                    v.push(f(Self::_idx_to_coordinate(idx as usize, size)));
                }
                v
            }
        }
    }

    pub fn new(size : Point2) -> Self where T : Clone + Default { Self::new_with_tile(size, T::default())}
    pub fn new_with_tile(mut size : Point2, tile : T) -> Self where T : Clone
    {
        size = size.max(Point2::ZERO);
        Self
        {
            size,
            tiles : 
            {
                let mut v = Vec::with_capacity(size.sum_axis().to_uint());
                for _idx in 0.. size.x * size.y
                {
                    v.push(tile.clone());
                }
                v
            }
        }
    }


    pub fn range_x(&self) -> Range<int> { 0..self.size.x }
    pub fn range_y(&self) -> Range<int> { 0..self.size.y }

    pub fn size (&self) -> Point2 { self.size }

    #[inline] pub fn is_inside_x(&self, x : impl ToInt) -> bool { let v = x.to_int(); v >= 0 && v < self.size.x }
    #[inline] pub fn is_inside_y(&self, y : impl ToInt) -> bool { let v = y.to_int(); v >= 0 && v < self.size.y }

    pub fn is_inside(&self, idx : Point2) -> bool { self.is_inside_x(idx.x) && self.is_inside_y(idx.y) }

    pub fn coordinate_to_idx(&self, idx : Point2) -> usize  { (idx.x+self.size.x*idx.y) as usize }
    pub fn idx_to_coordinate(&self, idx : usize ) -> Point2 { Self::_idx_to_coordinate(idx, self.size) }
    fn _idx_to_coordinate(idx : usize, size : Point2) -> Point2 { Point2::new(idx as isize % size.x , idx as isize / size.x) }

    pub fn try_get(&self, pos : Point2) -> Option<&T> 
    { 
        if self.is_inside(pos) { Some(&self.tiles[self.coordinate_to_idx(pos)]) } else { None }
    }
    pub fn try_get_mut(&mut self, pos : Point2) -> Option<&mut T> 
    {
        if self.is_inside(pos) { let idx = self.coordinate_to_idx(pos); Some(&mut self.tiles[idx]) } else { None }
    }

    /// panics if outside the grid
    pub fn get(&self, p : Point2) -> &T { self.try_get(p).unwrap() }
    pub fn get_mut(&mut self, p : Point2) -> &mut T { self.try_get_mut(p).unwrap() }

    pub fn swap(&mut self, a : Point2, b : Point2) 
    { 
        let a = self.coordinate_to_idx(a);
        let b = self.coordinate_to_idx(b);
        self.tiles.swap(a, b)
    }

    pub fn set(&mut self, val : T, idx : Point2) { self[idx] = val; }
    pub fn try_set(&mut self, val : T, idx : Point2) -> bool { if self.is_inside(idx) { self[idx] = val; true } else { false } }

    pub fn range_map(&mut self, p : Point2, size : Point2, action : fn(g : &mut Grid2<T>, p : Point2)) -> Rect2I
    {
        let mut p : Point2 = p.into();
        p = p.max(Point2::ZERO);

        let mut s : Point2 = size.into();
        s = s.min(self.size()-p);
        

        // Todo : do better
        for x in p.x..(p.x+s.x)
        {
            for y in p.y..(p.y+s.y)
            {
                action(self, point2(x, y));
            }
        }

        Rect2I::new(p, s)
    }

    pub fn try_range_set(&mut self, val : T, p : Point2, size : Point2) where T : Clone
    {
        let p : Point2 = p.into();
        let s : Point2 = size.into();

        // Todo : do better
        for x in p.x..(p.x+s.x)
        {
            for y in p.y..(p.y+s.y)
            {
                self.try_set(val.clone(), (x, y).into());
            }
        }
    }
}

impl<T> Index<Point2> for Grid2<T> { type Output=T; fn index(&self, index: Point2) -> &Self::Output { self.get(index.into()) }}
impl<T> IndexMut<Point2> for Grid2<T> { fn index_mut(&mut self, index: Point2) -> &mut Self::Output { self.get_mut(index.into()) }}

impl<T> Index<usize> for Grid2<T> { type Output=T; fn index(&self, index: usize) -> &Self::Output { &self.tiles[index] }}
impl<T> IndexMut<usize> for Grid2<T> { fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.tiles[index] }}

impl<T:Debug> Debug for Grid2<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result 
    {
        for y in (0..self.size.y).rev()
        {
            for x in 0..self.size.x
            {
                write!(f, "{:?} ", self.get(point2(x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl<T:Display> Display for Grid2<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result 
    {
        for y in (0..self.size.y).rev()
        {
            for x in 0..self.size.x
            {
                write!(f, "{} ", self.get(point2(x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}