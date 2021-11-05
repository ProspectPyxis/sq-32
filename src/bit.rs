use crate::error::BitError;
use std::mem;

pub fn get_first_on_bit_pos<T: Into<u64> + Copy>(b: T) -> Result<usize, BitError> {
    (0..mem::size_of::<T>())
        .find(|x| b.into() as usize & (1 << x) != 0)
        .ok_or(BitError::UnexpectedZero)
}

pub fn get_all_on_bits<T: Into<u64> + Copy>(b: T) -> Vec<usize> {
    let mut ons: Vec<usize> = Vec::new();

    for i in 0..mem::size_of::<T>() {
        if b.into() as usize & (1 << i) != 0 {
            ons.push(i);
        }
    }

    ons
}

pub fn is_bit_on<T: Into<u64>>(b: T, pos: u8) -> bool {
    b.into() & (1 << pos) != 0
}
