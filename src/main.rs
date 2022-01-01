// jkcoxson

fn main() {
    // Get arguments
    let args: Vec<String> = std::env::args().collect();
    let recursions = args[1].parse::<i32>().unwrap();
    let mut to_run = "echo \"RECURSION\" ".to_string();
    for _ in 0..recursions {
        to_run = format!("{} | cowsay -n ", to_run).to_string();
    }
    // Run the command
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(to_run)
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("failed to execute process");
    // Print the output
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
