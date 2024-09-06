use ark_ff::{
    fields::{MontBackend, MontConfig},
    FpConfig,
};

use ark_ff::{Fp64, PrimeField};
#[allow(warnings)]
use halo2_gadgets::poseidon::primitives::{
    self as poseidon1, ConstantLength, P128Pow5T3 as OrchardNullifier,
};

#[allow(warnings)]
use halo2_proofs::pasta::Fp;
use crate::merkle::merkle::LeafType;

struct P64MontConfig;
impl MontConfig<1> for P64MontConfig {
    const MODULUS: ark_ff::BigInt<1> = ark_ff::BigInt::new([18446744073709551615u64; 1]);
    const GENERATOR: ark_ff::Fp<MontBackend<Self, 1>, 1> = MontBackend::ONE;
    const TWO_ADIC_ROOT_OF_UNITY: ark_ff::Fp<MontBackend<Self, 1>, 1> =
        ark_ff::Fp::new(Self::MODULUS);
}

#[allow(warnings)]
pub type Fr64 = Fp64<MontBackend<P64MontConfig, 1>>;
pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

impl AsBytes for String {
    fn as_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl AsBytes for u64 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }
}

impl AsBytes for &[u8] {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

pub fn fr64_vec_to_u64(input: &Vec<u8>, input_2: &Vec<u8>) -> LeafType {
    let mut data = input.clone();
    data.extend(input_2);
    Fr64::from_be_bytes_mod_order(&data).0 .0[0]
}

pub fn hash_leaf<T1, T2>(a: T1, b: T2) -> LeafType
where
    T1: AsBytes,
    T2: AsBytes,
{
    fr64_vec_to_u64(&a.as_bytes(), &b.as_bytes())
}