pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u32::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl FromBytes for u64 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u64::from_be_bytes(bytes.try_into().unwrap())
    }
}
