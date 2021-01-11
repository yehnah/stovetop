use stovetop;

fn main() {
    stovetop::generate(
        "./example/template",
        "./example/stovetop.toml",
        "./output",
        None
    ).unwrap();
}
