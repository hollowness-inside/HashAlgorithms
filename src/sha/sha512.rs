use crate::HashBytes;

use super::common::Common;
use super::constants::{INIT_512, K_512};
use super::sha0::{Funcs, Sha};

pub type Sha512 = Sha<u64, 512, 80>;

impl Default for Sha512 {
    fn default() -> Self {
        Self {
            digest: INIT_512,
            konstants: K_512,

            block: [0; 16],
            funcs: Funcs {
                ch: Common::<u64>::ch,
                maj: Common::<u64>::maj,

                ls0: Common::<u64>::lowercase_sigma::<1, 8, 7>,
                ls1: Common::<u64>::lowercase_sigma::<19, 61, 6>,
                us0: Common::<u64>::uppercase_sigma::<28, 34, 39>,
                us1: Common::<u64>::uppercase_sigma::<14, 18, 41>,
            },
        }
    }
}

impl HashBytes for Sha512 {
    fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
        let mut hash = Sha512::default();
        hash.update(bytes);
        hash.digest()
    }
}
