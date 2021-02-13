use itertools::Itertools;
use variter::VarIter;

use crate::{
    card::{rank::Rank, Card},
    constants::{INT_RANKS, PRIMES},
};

#[derive(Copy, Clone)]
struct BitSequence {
    bits: i16,
    t: i16,
    next_bits: i16,
}

impl BitSequence {
    #[inline]
    const fn new(bits: i16) -> Self {
        Self {
            bits,
            t: 0,
            next_bits: 0,
        }
    }
}

impl Iterator for BitSequence {
    type Item = i16;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.t = self.bits | (self.bits.wrapping_sub(1));
        self.next_bits = self.t.wrapping_add(1)
            | (((!self.t & -!self.t) - 1) >> self.bits.trailing_zeros().wrapping_add(1));
        self.bits = self.next_bits;
        Some(self.next_bits)
    }
}

#[inline]
pub fn bit_sequence_generator(bits: i16) -> impl Iterator<Item = i16> { BitSequence::new(bits) }

#[inline]
pub fn combinations_generator<I>(iterable: I, r: usize) -> impl Iterator<Item = Vec<I::Item>>
where
    I: IntoIterator,
    I::Item: Clone,
{
    iterable.into_iter().combinations(r)
}

#[inline]
pub fn prime_product_from_rank_bits(rank_bits: i16) -> i32 {
    let mut product = 1;
    for i in INT_RANKS {
        if rank_bits & (1 << i) != 0 {
            product *= PRIMES[i as usize]
        }
    }
    product
}

#[inline]
pub fn prime_product_from_hand(hand: &[Card]) -> i32 {
    let mut product = 1;
    for &card in hand {
        product *= card.unique_integer() & 0xFF
    }
    product
}

#[inline]
/// Obtain the high card from a given set of rank bits bit-ORed together.
pub fn high_rank_from_rank_bits(rank_bits: i16) -> Rank {
    for i in INT_RANKS.rev() {
        if rank_bits & (1 << i) != 0 {
            // We don't want to return an Ace as the high card if it's a five-high straight
            return if i == 12 && (rank_bits & 0xF == 0b1111) {
                Rank::Five
            } else {
                Rank::ALL_VARIANTS[i as usize]
            };
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combinations_generator_works() {
        let combos = combinations_generator(vec!['c', 'a', 't'], 2).collect::<Vec<_>>();
        let expected_combos: [[char; 2]; 3] = [['c', 'a'], ['c', 't'], ['a', 't']];
        assert_eq!(combos.len(), expected_combos.len());
        for &combo in expected_combos.iter() {
            assert!(combos.contains(&combo.into()));
        }
    }

    #[test]
    fn bit_sequence_generator_works() {
        let some_number = 0b10011;
        let mut xs = bit_sequence_generator(some_number);
        let mut result = 0;

        let mut next_check = |bin_str: &'static str| {
            result = xs.next().unwrap();
            assert_eq!(result, i16::from_str_radix(bin_str, 2).unwrap());
        };

        next_check("00010101");
        next_check("00010110");
        next_check("00011001");
        next_check("00011010");
        next_check("00011100");
        next_check("00100011");
    }
}
