use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

// Enable or disable terminal raw mode(Unix-like systems)
fn set_raw_mode(enable: bool) {
    if cfg!(unix) {
        let mode = if enable { "raw" } else { "-raw" };
        let _ = Command::new("stty")
            .arg(mode)
            .stdin(Stdio::inherit())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

fn main() {
    let options = ["Status", "Sync", "Clean", "Exit"];
    let mut selected = 0usize;

    // Enter raw mode (no line buffering, instant key reads)
    set_raw_mode(true);
    print!("\x1b[?25l]"); // Hide cursor
    io::stdout().flush().unwrap();

    loop {
        // Clear screen and move cursor to top-left
        print!("\x1b[H\x1b[J");
        // print!("\x1b[J");
        println!("Use ↑ ↓ arrows to navigate, Enter to select:\n");

        // Draw menu
        for (i, opt) in options.iter().enumerate() {
            print!("\x1b[G");
            if i == selected {
                print!("\x1b[44;30m");
                println!("  {}", opt);
                print!("\x1b[0m");
            } else {
                println!("  {}", opt);
            }
        }
        io::stdout().flush().unwrap();

        // Read 3 bytes (enough for escape sequence like ESC [ A)
        let mut buffer = [0u8; 3];
        if let Ok(n) = io::stdin().read(&mut buffer) {
            if n == 0 {
                continue;
            }
            // println!("{} {} {}", buffer[0], buffer[1], buffer[2]);

            match buffer[0] {
                27 => {
                    // Escape sequences start with 27
                    if n >= 3 {
                        match buffer[2] {
                            65 => {
                                // Up Arrow
                                if selected > 0 {
                                    selected -= 1;
                                }
                            }
                            66 => {
                                // Down Arrow
                                if selected + 1 < options.len() {
                                    selected += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                107 | 119 => { // k | w
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                106 | 115 => { // j | s
                    if selected + 1 < options.len() {
                        selected += 1;
                    }
                }
                10 | 13 | 32 => { 
                    // Enter key (\n or \r) or Spacebar
                    break;
                }
                _ => {}
            }
        }
    }

    // Restore terminal state
    set_raw_mode(false);
    // print!("\x1b[H\x1b[>25h"); // Show cursor again
    print!("\x1b[>25h"); // Show cursor again
    println!("\nSelected: {}", options[selected]);
}
