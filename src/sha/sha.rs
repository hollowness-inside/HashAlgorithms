pub struct Sha<T, const DIGEST_SIZE: usize, const BLOCK_SIZE: usize> {
    pub(super) digest: [T; DIGEST_SIZE],
    pub(super) block: [T; BLOCK_SIZE],
}
