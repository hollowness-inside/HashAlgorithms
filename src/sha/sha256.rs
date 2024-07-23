use crate::HashBytes;

use super::common::Common;
use super::constants::{INIT_256, K_256};
use super::sha::{Funcs, Sha};

pub type Sha256 = Sha<u32, 256, 64>;

impl Default for Sha256 {
    fn default() -> Self {
        Self {
            digest: INIT_256,
            konstants: K_256,

            block: [0; 16],
            funcs: Funcs {
                ch: Common::<u32>::ch,
                maj: Common::<u32>::maj,

                ls0: Common::<u32>::lowercase_sigma::<7, 18, 3>,
                ls1: Common::<u32>::lowercase_sigma::<17, 19, 10>,
                us0: Common::<u32>::uppercase_sigma::<2, 13, 22>,
                us1: Common::<u32>::uppercase_sigma::<6, 11, 25>,
            },
        }
    }
}

impl HashBytes for Sha256 {
    fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
        let mut hash = Sha256::default();
        hash.update(bytes);
        hash.digest()
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
