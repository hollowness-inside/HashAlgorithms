fn main() {
    let digest = HashAlgorithms::sha::sha512(b"Hello World");
    let hex: String = digest.into_iter().map(|byte| format!("{byte:X}")).collect();

    println!("{hex}");
}
