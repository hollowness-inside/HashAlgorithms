use crate::HashBytes;

use super::common::Common;
use super::constants::{INIT_256, K_256};
use super::sha0::{Funcs, Sha};

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
