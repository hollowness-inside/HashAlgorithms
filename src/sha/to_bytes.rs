pub trait ToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

impl ToBytes for u32 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ToBytes for u64 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}
