use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::api::ManagedTypeApi;

use super::{BigFloat, ManagedType};

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait for BigFloat<M> {
            type Output = BigFloat<M>;

            fn $method(self, other: BigFloat<M>) -> BigFloat<M> {
                let api = M::instance();
                api.$api_func(self.handle, self.handle, other.handle);
                BigFloat::from_raw_handle(self.handle)
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigFloat<M>> for &'a BigFloat<M> {
            type Output = BigFloat<M>;

            fn $method(self, other: &BigFloat<M>) -> BigFloat<M> {
                let api = M::instance();
                let result = api.bf_new_zero();
                api.$api_func(result, self.handle, other.handle);
                BigFloat::from_raw_handle(result)
            }
        }
    };
}

binary_operator! {Add, add, bf_add}
binary_operator! {Sub, sub, bf_sub}
binary_operator! {Mul, mul, bf_mul}
binary_operator! {Div, div, bf_div}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<BigFloat<M>> for BigFloat<M> {
            #[inline]
            fn $method(&mut self, other: Self) {
                let api = M::instance();
                api.$api_func(self.handle, self.handle, other.handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigFloat<M>> for BigFloat<M> {
            #[inline]
            fn $method(&mut self, other: &BigFloat<M>) {
                let api = M::instance();
                api.$api_func(self.handle, self.handle, other.handle);
            }
        }
    };
}

binary_assign_operator! {AddAssign, add_assign, bf_add}
binary_assign_operator! {SubAssign, sub_assign, bf_sub}
binary_assign_operator! {MulAssign, mul_assign, bf_mul}
binary_assign_operator! {DivAssign, div_assign, bf_div}

impl<M: ManagedTypeApi> Neg for BigFloat<M> {
    type Output = BigFloat<M>;

    fn neg(self) -> Self::Output {
        let api = M::instance();
        let result = api.bf_new_zero();
        api.bf_neg(result, self.handle);
        BigFloat::from_raw_handle(result)
    }
}