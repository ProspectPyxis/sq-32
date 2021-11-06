use crate::error::BitError;
use std::mem::size_of;

pub fn first_on_pos<T: Into<u64> + Copy>(b: T) -> Result<u8, BitError> {
    (0..(size_of::<T>() as u8))
        .find(|x| b.into() as usize & (1 << x) != 0)
        .ok_or(BitError::UnexpectedZero)
}

pub fn all_on_bits<T: Into<u64> + Copy>(b: T) -> Vec<u8> {
    let mut ons: Vec<u8> = Vec::new();

    for i in 0..(size_of::<T>() as u8) {
        if b.into() & (1 << i) != 0 {
            ons.push(i);
        }
    }

    ons
}

pub fn is_pos_on<T: Into<u64>>(b: T, pos: u8) -> bool {
    b.into() & (1 << pos) != 0
}
