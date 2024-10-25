mod timer;
use std::io::Write;

// Prompt to Shell
fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn main() {
    let mut counter = 0;

    loop {
        let input = prompt("< Tomato-Timer: (p)omodoro | (b)reak | (e)xit >");
        match input.as_str() {
            "p" | "pomodoro" => {
                timer::productive_time();
                counter += 1;
            }

            "b" | "break" => {
                if (counter % 4) == 0 {
                    timer::break_time(true);
                } else {
                    timer::break_time(false);
                }
            }

            "e" | "exit" => {
                break;
            }

            _ => println!("Invalid Command"),
        }
    }
}
