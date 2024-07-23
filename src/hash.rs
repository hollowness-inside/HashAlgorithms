pub trait HashBytes {
    fn hash_bytes(bytes: &[u8]) -> Vec<u8>;
}