use stovetop::{ generate };

fn main() {
    let files = generate(
        "./example/template",
        "./example/stovetop.toml",
        "./output",
        None
    );
    println!("{:?}", files);
}
