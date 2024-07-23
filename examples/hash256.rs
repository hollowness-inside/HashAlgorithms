use HashAlgorithms::HashBytes;

fn main() {
    let digest = HashAlgorithms::sha::Sha256::hash_bytes(b"Hello World");
    let hex: String = digest.into_iter().map(|byte| format!("{byte:X}")).collect();

    println!("{hex}");
}
