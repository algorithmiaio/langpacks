use cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/lib.cpp")
        .compile("algo");
}
