#![feature(const_fn)]
#![feature(panic_implementation)]
#![feature(lang_items)]
#![cfg_attr(feature = "alloc", feature (alloc, alloc_system, use_extern_macros))]
#![no_std]
#![cfg_attr(feature = "math", feature(core_float))]

#[cfg(feature = "alloc")]
extern crate alloc as core_alloc;
#[cfg(feature = "alloc")]
extern crate alloc_system;

#[cfg(feature = "alloc")]
#[global_allocator]
static A: alloc_system::System = alloc_system::System;

extern crate arrayvec;
extern crate gcn;

pub mod game;
pub mod link;
pub mod system;
pub mod warping;
mod lang_items;

pub type Addr = usize;
pub use link::Link;

use core::fmt;

#[repr(C, packed)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coord {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

impl Clone for Coord {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { write!(f, "{:.2}, {:.2}, {:.2}", self.x, self.y, self.z) }
    }
}

#[cfg(feature = "alloc")]
pub mod alloc {
    pub use core_alloc::vec::Vec;
    pub use core_alloc::boxed::Box;
    pub use core_alloc::string::String;
    pub use core_alloc::format;
}

