use stovetop::*;

fn main() {
    let files = generate(
        "./example",
        "./example/template.toml",
        "./output",
        "./example",
    );
    println!("{:?}", files);
}
