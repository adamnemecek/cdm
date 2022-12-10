use crate::{
    group::AbelianGroup,
    identity::{Identity, Multiplication},
    Natural,
};

/// Multiplication must be:
/// - Associative
/// - Distributive over addition
pub trait Ring: AbelianGroup + std::ops::Mul<Output = Self> + Identity<Multiplication> {
    /// The identity element for multiplication
    fn one() -> Self {
        <Self as Identity<Multiplication>>::identity()
    }
    fn is_one(&self) -> bool {
        &Self::one() == self
    }
    /// The multiplicative inverse in the case that `self` is a unit
    fn multiplicative_inverse(&self) -> Option<Self>;
    fn pow(&self, pow: Natural) -> Self
    where
        Self: Clone,
    {
        let mut total = Ring::one();
        for _ in 0..u128::from(pow) {
            total = total * self.clone();
        }
        total
    }
}
