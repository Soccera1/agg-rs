fn main() {
    cc::Build::new()
        .file("src/comp.rs")
        .compile("agg-rs");
}
