use super::wrapadd::WrappingAdd;

pub(super) struct Funcs<T> {
    pub(super) ch: fn(a: T, b: T, c: T) -> T,
    pub(super) maj: fn(a: T, b: T, c: T) -> T,

    pub(super) ls0: fn(x: T) -> T,
    pub(super) ls1: fn(x: T) -> T,
    pub(super) us0: fn(x: T) -> T,
    pub(super) us1: fn(x: T) -> T,
}

pub struct Sha<T, const DIGEST_SIZE: usize, const BLOCK_SIZE: usize, const ROUNDS: usize> {
    pub(super) digest: [T; DIGEST_SIZE],
    pub(super) block: [T; BLOCK_SIZE],
    pub(super) funcs: Funcs<T>,
    pub(super) konstants: [T; ROUNDS],
}

impl<T, const DIGEST_SIZE: usize, const BLOCK_SIZE: usize, const ROUNDS: usize>
    Sha<T, DIGEST_SIZE, BLOCK_SIZE, ROUNDS>
where
    T: Default + Copy + WrappingAdd,
{
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
