fn main() {
    println!("{}", concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\", "target\\debug"));
    engine::test::print_int(42);
}