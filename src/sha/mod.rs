const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const INIT: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub fn sha256(messsage: &[u8]) -> Vec<u8> {
    let mut hash_values: [u32; 8] = INIT;

    let blocks = preprocess_256(messsage);

    for block in blocks.iter() {
        let mut schedule: Vec<u32> = vec![0; 64];

        for i in 0..16 {
            schedule[i] = block[i];
        }

        for t in 16..64 {
            let x = lowercase_sigma_1(schedule[t - 2])
                .wrapping_add(schedule[t - 7])
                .wrapping_add(lowercase_sigma_0(schedule[t - 15]))
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

        for t in 0..64 {
            let t1 = h
                .wrapping_add(uppercase_sigma_1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[t])
                .wrapping_add(schedule[t]);

            let t2 = uppercase_sigma_0(a).wrapping_add(maj(a, b, c));

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

fn preprocess_256(messsage: &[u8]) -> Vec<Vec<u32>> {
    pad_256(&messsage)
        .chunks_exact(64)
        .map(|chunk| {
            chunk
                .chunks_exact(4)
                .map(|int| u32::from_be_bytes(int.try_into().unwrap()))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn pad_256(data: &[u8]) -> Vec<u8> {
    let bits_len = data.len() * 8;

    let mut data = data.to_vec();
    data.push(0x80);

    while (data.len() * 8 + 64) % 512 != 0 {
        data.push(0);
    }

    data.extend(bits_len.to_be_bytes());

    data
}

const fn uppercase_sigma_0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

const fn uppercase_sigma_1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

const fn lowercase_sigma_0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
}

const fn lowercase_sigma_1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10)
}

const fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

const fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

const fn rotr(x: u32, n: u32) -> u32 {
    (x >> n) | (x << (<u32>::BITS - n))
}

#[cfg(test)]
mod tests {
    use super::sha256;

    macro_rules! hash_expect {
        ($inp: literal, $exp: literal) => {
            let digest = sha256($inp);
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
