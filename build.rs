fn main() {
    cc::Build::new()
        .file("src/comp.c")
        .compile("agg-rs");
}
