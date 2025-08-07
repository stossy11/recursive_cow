// jkcoxson
// + stossy11

fn cowsay_native(message: &str) -> Vec<String> {
    let length = message.lines().map(|l| l.len()).max().unwrap_or(0);
    let top = format!(" {}", "-".repeat(length + 2));
    let bottom = top.clone();

    let lines: Vec<&str> = message.lines().collect();
    let middle: Vec<String> = if lines.len() > 1 {
        lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    format!("/ {:width$} \\", line, width = length)
                } else if i == lines.len() - 1 {
                    format!("\\ {:width$} /", line, width = length)
                } else {
                    format!("| {:width$} |", line, width = length)
                }
            })
            .collect()
    } else {
        vec![format!("< {:width$} >", message, width = length)]
    };


    let cow = vec![
        String::from("        \\   ^__^"),
        String::from("         \\  (oo)\\_______"),
        String::from("            (__)\\       )\\/\\"),
        String::from("                ||----w |"),
        String::from("                ||     ||"),
    ];

    let mut result = Vec::new();
    result.push(top);
    result.extend(middle);
    result.push(bottom);
    result.extend(cow);
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let recursions = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);

    let mut current_output = vec!["RECURSION".to_string()];

    for _ in 0..recursions {
        let input = current_output.join("\n");
        current_output = cowsay_native(&input);
    }

    for line in current_output {
        println!("{}", line);
    }
}
