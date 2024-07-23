use super::common::{pad, Common};

type Func = Common<u64>;

const K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

const INIT: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

pub fn sha512(messsage: &[u8]) -> Vec<u8> {
    let mut hash_values = INIT;

    let blocks = preprocess_512(messsage);

    for block in blocks.iter() {
        let mut schedule: Vec<u64> = vec![0; 80];
        schedule[..16].copy_from_slice(&block[..16]);

        for t in 16..80 {
            let x = Func::lowercase_sigma::<19, 61, 6>(schedule[t - 2])
                .wrapping_add(schedule[t - 7])
                .wrapping_add(Func::lowercase_sigma::<1, 8, 7>(schedule[t - 15]))
                .wrapping_add(schedule[t - 16]);

            schedule[t] = x;
        }

        let mut a = hash_values[0];
        let mut b = hash_values[1];
        let mut c = hash_values[2];
        let mut d = hash_values[3];
        let mut e = hash_values[4];
        let mut f = hash_values[5];
        let mut g = hash_values[6];
        let mut h = hash_values[7];

        for t in 0..80 {
            let t1 = h
                .wrapping_add(Func::uppercase_sigma::<14, 18, 41>(e))
                .wrapping_add(Func::ch(e, f, g))
                .wrapping_add(K[t])
                .wrapping_add(schedule[t]);

            let t2 = Func::uppercase_sigma::<28, 34, 39>(a).wrapping_add(Func::maj(a, b, c));

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        hash_values[0] = a.wrapping_add(hash_values[0]);
        hash_values[1] = b.wrapping_add(hash_values[1]);
        hash_values[2] = c.wrapping_add(hash_values[2]);
        hash_values[3] = d.wrapping_add(hash_values[3]);
        hash_values[4] = e.wrapping_add(hash_values[4]);
        hash_values[5] = f.wrapping_add(hash_values[5]);
        hash_values[6] = g.wrapping_add(hash_values[6]);
        hash_values[7] = h.wrapping_add(hash_values[7]);
    }

    hash_values
        .into_iter()
        .flat_map(|value| value.to_be_bytes())
        .collect()
}

fn preprocess_512(messsage: &[u8]) -> Vec<Vec<u64>> {
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

#[cfg(test)]
mod tests {
    use super::sha512;

    macro_rules! hash_expect {
        ($inp: literal, $exp: literal) => {
            let digest = sha512($inp);
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
