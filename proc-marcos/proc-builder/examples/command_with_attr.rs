use proc_builder::BuilderWithAttr;

#[derive(Debug, BuilderWithAttr)]
struct Command {
    executable: String,
    #[builder(each = "arg", default = "Default::default()")]
    args: Vec<String>,
    #[builder(default = "vec![]")]
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("find")
        .arg("-c")
        .arg("-vvv")
        // .env("RUST_LOG=info")
        .current_dir("/Us")
        .finish()
        .unwrap();
    println!("{:?}", command);

    let command = CommandBuilder::default();
    println!("{:?}", command);
}