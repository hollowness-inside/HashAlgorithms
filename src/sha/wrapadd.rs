pub trait WrappingAdd {
    fn wrapping_add(self, other: Self) -> Self;
}

impl WrappingAdd for u32 {
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl WrappingAdd for u64 {
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}
