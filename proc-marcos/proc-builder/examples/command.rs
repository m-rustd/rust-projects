use proc_builder::Builder;

#[derive(Debug, Builder)]
struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("ls")
        .args(vec!["-l".to_string()])
        .env(vec!["FOO=BAR".to_string()])
        .current_dir("/tmp")
        .finish()
        .unwrap();
    println!("{:?}", command);

    let command = CommandBuilder::default();
    println!("{:?}", command);
}