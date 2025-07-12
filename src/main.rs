extern "C" {
    fn c_main() -> i32;
}

fn main() {
    let code = unsafe { c_main() };
    std::process::exit(code);
}
