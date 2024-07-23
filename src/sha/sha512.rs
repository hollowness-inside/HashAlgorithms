use super::common::{pad, Common};
use super::constants::{INIT_512, K_512};

type Func = Common<u64>;

pub fn sha512(messsage: &[u8]) -> Vec<u8> {
    let mut hash_values = INIT_512;
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
                .wrapping_add(K_512[t])
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
