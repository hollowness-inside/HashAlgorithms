mod common;
mod constants;

mod from_bytes;
mod to_bytes;
mod wrapadd;

mod sha;
mod sha256;
mod sha512;

use std::marker::PhantomData;

pub use sha256::Sha256;
pub use sha512::Sha512;

pub struct Sha<T> {
    _t: PhantomData<T>,
}
