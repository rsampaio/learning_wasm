use std::process::Command;

fn main() {
    println!("Hello, world!");
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .ok()
        .expect("error executing command");
    let encoded = String::from_utf8_lossy(output.stdout.as_slice());
    println!("> {}", encoded);
}
