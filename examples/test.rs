use hash_algorithms::sha::Sha256;

fn main() {
    let mut hasher = Sha256::default();

    let feed: &[u32; 4] = &[123, 23, 4, 55];
    hasher.update_block(feed);

    let digest = hasher.digest();
    println!("{digest:?}");
}
