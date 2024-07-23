use super::utils::{FromBytes, ToBytes, WrappingAdd};

pub(super) struct Funcs<T> {
    pub(super) ch: fn(a: T, b: T, c: T) -> T,
    pub(super) maj: fn(a: T, b: T, c: T) -> T,

    pub(super) ls0: fn(x: T) -> T,
    pub(super) ls1: fn(x: T) -> T,
    pub(super) us0: fn(x: T) -> T,
    pub(super) us1: fn(x: T) -> T,
}

pub struct Sha<T, const HASHSIZE: usize, const ROUNDS: usize> {
    pub(super) digest: [T; 8],
    pub(super) block: [T; 16],
    pub(super) funcs: Funcs<T>,
    pub(super) konstants: [T; ROUNDS],
}

impl<T, const HASHSIZE: usize, const ROUNDS: usize> Sha<T, HASHSIZE, ROUNDS>
where
    T: Default + Copy + WrappingAdd + FromBytes + ToBytes,
{
    pub fn update(&mut self, bytes: &[u8]) {
        for block in Self::preprocess(bytes).iter() {
            self.block.copy_from_slice(block);
            self.calculate_block();
        }
    }

    pub fn digest(&self) -> Vec<u8> {
        self.digest
            .into_iter()
            .flat_map(|value| value.to_bytes())
            .collect()
    }

    pub(super) fn preprocess(messsage: &[u8]) -> Vec<Vec<T>> {
        let x = Self::pad(HASHSIZE * 2, messsage)
            .chunks_exact(HASHSIZE / 4)
            .map(|chunk| {
                chunk
                    .chunks_exact(HASHSIZE / 64)
                    .map(|int| T::from_bytes(int))
                    .collect::<Vec<T>>()
            })
            .collect();
        x
    }

    pub(super) fn pad(n: usize, data: &[u8]) -> Vec<u8> {
        let bits_len = data.len() * 8;

        let mut data = data.to_vec();
        data.push(0x80);

        while (data.len() * 8 + 64) % n != 0 {
            data.push(0);
        }

        data.extend(bits_len.to_be_bytes());
        data
    }

    pub(super) fn calculate_schedule(&self) -> [T; ROUNDS] {
        let mut schedule = [T::default(); ROUNDS];
        schedule[..16].copy_from_slice(&self.block[..16]);

        for t in 16..ROUNDS {
            let x = (self.funcs.ls1)(schedule[t - 2])
                .wrapping_add(schedule[t - 7])
                .wrapping_add((self.funcs.ls0)(schedule[t - 15]))
                .wrapping_add(schedule[t - 16]);

            schedule[t] = x;
        }

        schedule
    }

    pub(super) fn calculate_block(&mut self) {
        let schedule = self.calculate_schedule();

        let mut a = self.digest[0];
        let mut b = self.digest[1];
        let mut c = self.digest[2];
        let mut d = self.digest[3];
        let mut e = self.digest[4];
        let mut f = self.digest[5];
        let mut g = self.digest[6];
        let mut h = self.digest[7];

        for t in 0..ROUNDS {
            let t1 = h
                .wrapping_add((self.funcs.us1)(e))
                .wrapping_add((self.funcs.ch)(e, f, g))
                .wrapping_add(self.konstants[t])
                .wrapping_add(schedule[t]);

            let t2 = (self.funcs.us0)(a).wrapping_add((self.funcs.maj)(a, b, c));

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        self.digest[0] = a.wrapping_add(self.digest[0]);
        self.digest[1] = b.wrapping_add(self.digest[1]);
        self.digest[2] = c.wrapping_add(self.digest[2]);
        self.digest[3] = d.wrapping_add(self.digest[3]);
        self.digest[4] = e.wrapping_add(self.digest[4]);
        self.digest[5] = f.wrapping_add(self.digest[5]);
        self.digest[6] = g.wrapping_add(self.digest[6]);
        self.digest[7] = h.wrapping_add(self.digest[7]);
    }
}
