pub trait HashBytes {
    fn hash_bytes(&[u8]) -> Vec<u8>;
}