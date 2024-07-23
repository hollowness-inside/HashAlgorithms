use crate::HashBytes;

use super::common::{pad, Common};
use super::constants::{INIT_256, K_256};
use super::sha::{Funcs, Sha};

type Func = Common<u32>;

pub type Sha256 = Sha<u32, 8, 16, 64>;

impl Default for Sha256 {
    fn default() -> Self {
        Self {
            digest: INIT_256,
            konstants: K_256,

            block: [0; 16],
            funcs: Funcs {
                ch: Func::ch,
                maj: Func::maj,

                ls0: Func::lowercase_sigma::<7, 18, 3>,
                ls1: Func::lowercase_sigma::<17, 19, 10>,
                us0: Func::uppercase_sigma::<2, 13, 22>,
                us1: Func::uppercase_sigma::<6, 11, 25>,
            },
        }
    }
}

impl HashBytes for Sha256 {
    fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
        let mut hash = Sha256::default();
        hash.update(bytes)
    }
}

impl Sha256 {
    fn update(&mut self, bytes: &[u8]) -> Vec<u8> {
        for block in Self::preprocess(bytes).iter() {
            self.block.copy_from_slice(block);
            self.calculate_block();
        }

        self.digest
            .into_iter()
            .flat_map(|value| value.to_be_bytes())
            .collect()
    }

    fn preprocess(messsage: &[u8]) -> Vec<Vec<u32>> {
        pad::<512>(messsage)
            .chunks_exact(64)
            .map(|chunk| {
                chunk
                    .chunks_exact(4)
                    .map(|int| u32::from_be_bytes(int.try_into().unwrap()))
                    .collect::<Vec<u32>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Sha256;
    use crate::HashBytes;

    macro_rules! hash_expect {
        ($inp: literal, $exp: literal) => {
            let digest = Sha256::hash_bytes($inp);
            let hex: String = digest
                .into_iter()
                .map(|byte| format!("{byte:02X}"))
                .collect();
            assert_eq!(hex, $exp);
        };
    }

    #[test]
    fn hash256() {
        hash_expect!(
            b"",
            "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855"
        );
        hash_expect!(
            b"a",
            "CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB"
        );
        hash_expect!(
            b"Hello World",
            "A591A6D40BF420404A011733CFB7B190D62C65BF0BCDA32B57B277D9AD9F146E"
        );
        hash_expect!(
            b"Lorem Ipsum",
            "030DC1F936C3415AFF3F3357163515190D347A28E758E1F717D17BAE453541C9"
        );
    }
}
