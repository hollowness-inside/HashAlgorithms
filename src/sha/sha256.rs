use crate::HashBytes;

use super::common::{pad, Common};
use super::constants::{INIT_256, K_256};
use super::sha::Sha;

type Func = Common<u32>;

pub type Sha256 = Sha<u32>;

impl HashBytes for Sha256 {
    fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
        let mut digest = INIT_256;
        let blocks = preprocess_256(bytes);

        for block in blocks.iter() {
            let mut schedule: Vec<u32> = vec![0; 64];
            schedule[..16].copy_from_slice(&block[..16]);

            for t in 16..64 {
                let x = Func::lowercase_sigma::<17, 19, 10>(schedule[t - 2])
                    .wrapping_add(schedule[t - 7])
                    .wrapping_add(Func::lowercase_sigma::<7, 18, 3>(schedule[t - 15]))
                    .wrapping_add(schedule[t - 16]);

                schedule[t] = x;
            }

            let mut a = digest[0];
            let mut b = digest[1];
            let mut c = digest[2];
            let mut d = digest[3];
            let mut e = digest[4];
            let mut f = digest[5];
            let mut g = digest[6];
            let mut h = digest[7];

            for t in 0..64 {
                let t1 = h
                    .wrapping_add(Func::uppercase_sigma::<6, 11, 25>(e))
                    .wrapping_add(Func::ch(e, f, g))
                    .wrapping_add(K_256[t])
                    .wrapping_add(schedule[t]);

                let t2 = Func::uppercase_sigma::<2, 13, 22>(a).wrapping_add(Func::maj(a, b, c));

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }

            digest[0] = a.wrapping_add(digest[0]);
            digest[1] = b.wrapping_add(digest[1]);
            digest[2] = c.wrapping_add(digest[2]);
            digest[3] = d.wrapping_add(digest[3]);
            digest[4] = e.wrapping_add(digest[4]);
            digest[5] = f.wrapping_add(digest[5]);
            digest[6] = g.wrapping_add(digest[6]);
            digest[7] = h.wrapping_add(digest[7]);
        }

        digest
            .into_iter()
            .flat_map(|value| value.to_be_bytes())
            .collect()
    }
}

fn preprocess_256(messsage: &[u8]) -> Vec<Vec<u32>> {
    pad::<512>(&messsage)
        .chunks_exact(64)
        .map(|chunk| {
            chunk
                .chunks_exact(4)
                .map(|int| u32::from_be_bytes(int.try_into().unwrap()))
                .collect::<Vec<u32>>()
        })
        .collect()
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
