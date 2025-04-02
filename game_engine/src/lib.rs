#![allow(unused_imports)]
#![allow(dead_code)]

use std::{marker::PhantomData, ops::Deref};

pub use math::*;
pub use util::*;

/// Module dedicated to wrap the current librairy feature (macroquad)
pub mod wrapper;
pub use wrapper::*;

pub mod context;
pub use context::*;

pub mod graphics;
pub use graphics::*;

pub mod game;
pub use game::*;

pub mod input;
pub use input::*;