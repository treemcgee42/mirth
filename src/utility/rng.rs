//! A thread-local reproducible pseudo random number generator.
//!
//! In a multithreaded context, one struct should be passed to each thread.
//! A construction method can deterministically produce these. Thus, 
//! a program using these generators is only reproducible when using the same 
//! number of threads.

use rand_core::RngCore;
use rand_pcg;

use crate::config::{Float, KindOfFloatCheckable, KindOfFloat};

pub struct RandomNumberGenerator {
    internal: rand_pcg::Pcg64Mcg,
}

impl RandomNumberGenerator {
    pub fn from_seed(seed: u32) -> Self {
        RandomNumberGenerator {
            internal: rand_pcg::Pcg64Mcg::new(seed as u128),
        }
    }

    pub fn next_float(&mut self) -> Float {
        match Float::kind() {
            KindOfFloat::Float32 => {
                let random_int = self.internal.next_u32();
                (random_int as Float) / (u32::MAX as Float)
            },
            KindOfFloat::Float64 => {
                let random_int = self.internal.next_u64();
                (random_int as Float) / (u64::MAX as Float)
            },
        }
    } 
}

