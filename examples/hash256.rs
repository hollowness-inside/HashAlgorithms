use hash_algorithms::sha::Sha256;
use hash_algorithms::HashBytes;

fn main() {
    let digest = Sha256::hash_bytes(b"Hello World");
    let hex: String = digest.into_iter().map(|byte| format!("{byte:X}")).collect();

    println!("{hex}");
}
