use std::marker::PhantomData;

macro_rules! impl_common {
    ($typ: ident) => {
        impl Common<$typ> {
            pub(super) const fn uppercase_sigma<const A: $typ, const B: $typ, const C: $typ>(
                x: $typ,
            ) -> $typ {
                Self::rotr(x, A) ^ Self::rotr(x, B) ^ Self::rotr(x, C)
            }

            pub(super) const fn lowercase_sigma<const A: $typ, const B: $typ, const C: $typ>(
                x: $typ,
            ) -> $typ {
                Self::rotr(x, A) ^ Self::rotr(x, B) ^ (x >> C)
            }

            pub(super) const fn ch(x: $typ, y: $typ, z: $typ) -> $typ {
                (x & y) ^ (!x & z)
            }

            pub(super) const fn maj(x: $typ, y: $typ, z: $typ) -> $typ {
                (x & y) ^ (x & z) ^ (y & z)
            }

            pub(super) const fn rotr(x: $typ, n: $typ) -> $typ {
                (x >> n) | (x << (<$typ>::BITS as $typ - n))
            }
        }
    };
}

pub(super) struct Common<T> {
    _id: PhantomData<T>,
}

impl_common!(u32);
impl_common!(u64);

pub(super) fn pad<const N: usize>(data: &[u8]) -> Vec<u8> {
    let bits_len = data.len() * 8;

    let mut data = data.to_vec();
    data.push(0x80);

    while (data.len() * 8 + 64) % N != 0 {
        data.push(0);
    }

    data.extend(bits_len.to_be_bytes());
    data
}
