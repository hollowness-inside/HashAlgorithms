use hash_algorithms::sha::Sha512;
use hash_algorithms::HashBytes;

fn main() {
    let digest = Sha512::hash_bytes(b"Hello World");
    let hex: String = digest.into_iter().map(|byte| format!("{byte:X}")).collect();

    println!("{hex}");
}
