use crate::HashBytes;

use super::common::{pad, Common};
use super::constants::{INIT_512, K_512};
use super::sha::{Funcs, Sha};

type Func = Common<u64>;

pub type Sha512 = Sha<u64, 8, 16, 80>;

impl Default for Sha512 {
    fn default() -> Self {
        Self {
            digest: INIT_512,
            konstants: K_512,

            block: [0; 16],
            funcs: Funcs {
                ch: Func::ch,
                maj: Func::maj,

                ls0: Func::lowercase_sigma::<1, 8, 7>,
                ls1: Func::lowercase_sigma::<19, 61, 6>,
                us0: Func::uppercase_sigma::<28, 34, 39>,
                us1: Func::uppercase_sigma::<14, 18, 41>,
            },
        }
    }
}

impl HashBytes for Sha512 {
    fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
        let mut hash = Sha512::default();
        hash.update(bytes)
    }
}

impl Sha512 {
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

    fn preprocess(messsage: &[u8]) -> Vec<Vec<u64>> {
        let x = pad::<1024>(messsage)
            .chunks_exact(128)
            .map(|chunk| {
                chunk
                    .chunks_exact(8)
                    .map(|int| u64::from_be_bytes(int.try_into().unwrap()))
                    .collect::<Vec<u64>>()
            })
            .collect();
        x
    }
}


#[cfg(test)]
mod tests {
    use super::Sha512;
    use crate::HashBytes;

    macro_rules! hash_expect {
        ($inp: literal, $exp: literal) => {
            let digest = Sha512::hash_bytes($inp);
            let hex: String = digest
                .into_iter()
                .map(|byte| format!("{byte:02X}"))
                .collect();
            assert_eq!(hex, $exp);
        };
    }

    #[test]
    fn hash512() {
        hash_expect!(
            b"",
            "CF83E1357EEFB8BDF1542850D66D8007D620E4050B5715DC83F4A921D36CE9CE47D0D13C5D85F2B0FF8318D2877EEC2F63B931BD47417A81A538327AF927DA3E"
        );
        hash_expect!(
            b"a",
            "1F40FC92DA241694750979EE6CF582F2D5D7D28E18335DE05ABC54D0560E0F5302860C652BF08D560252AA5E74210546F369FBBBCE8C12CFC7957B2652FE9A75"
        );
        hash_expect!(
            b"Hello World",
            "2C74FD17EDAFD80E8447B0D46741EE243B7EB74DD2149A0AB1B9246FB30382F27E853D8585719E0E67CBDA0DAA8F51671064615D645AE27ACB15BFB1447F459B"
        );
        hash_expect!(
            b"Lorem Ipsum",
            "7FFB69027702D73E3376DE17B1377C29EB61A5510BC6196B5A251DC83EF1B444E98138C0F60727BA0E945A62AF0715AE5BB4A6D7435EF1BD8184C7C7C158F317"
        );
    }
}
