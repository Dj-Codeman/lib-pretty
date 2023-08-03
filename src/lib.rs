// Just a hand full of random colors 
const COLOR_BLACK:  &str = "\u{001b}[30m"; // Terminals are black why output black ???
const COLOR_RED:    &str = "\u{001b}[31m";
const COLOR_GREEN:  &str = "\u{001b}[32m";
const COLOR_YELLOW: &str = "\u{001b}[33m";
const COLOR_BLUE:   &str = "\u{001b}[34m";
const COLOR_BOLD:   &str = "\x1B[1m";
const COLOR_ITAL:   &str = "\x1B[3m";
const COLOR_RESET:  &str = "\u{001b}[0m";

/// Function take a color and a text reference and outputs to the terminal in the color
pub fn output(color: &str, text: &str) {

    match color {
        "RED" => {
            let color: &str = COLOR_RED;
            print_text(color, &text);
        }
        "GREEN" => {
            let color: &str = COLOR_GREEN;
            print_text(color, &text);
        }
        "YELLOW" => {
            let color: &str = COLOR_YELLOW;
            print_text(color, &text);
        }
        "BLUE" => {
            let color: &str = COLOR_BLUE;
            print_text(color, &text);        
        }
        _ => {
            let color: &str = COLOR_BLACK;
            print_text(color, &text);
        }
    }

    fn print_text(color: &str, text: &str) {
        print!("{}{}{}{}\n", COLOR_BOLD, color, text, COLOR_RESET);

    }
}

/// Important message

/// Prepends `Notice` to the text and italicies
pub fn notice(text: &str) {
    println!("{}{}Notice: {}! {}", COLOR_BOLD, COLOR_BLUE, text, COLOR_RESET);
}

/// Prepends `Warning` to the text and italicies
// just a simple style change to convery importance to users
pub fn warn(text: &str) {
    println!("{}{}Warning: {}! {}\n", COLOR_BOLD, COLOR_YELLOW, text, COLOR_RESET);
}

/// Exit messages

/// pass makes the texxt color green and exits with a 0 status code 
pub fn pass(text: &str) {
    println!("Exiting: {}{}{}{}! {}", COLOR_BOLD, COLOR_ITAL, COLOR_GREEN, text, COLOR_RESET);
    std::process::exit(0);
}

/// halt makes the texxt color red and exits with a 255 status code 
pub fn halt(text: &str) {
    println!("{}{}Panic!: {}! {}", COLOR_BOLD, COLOR_RED, text, COLOR_RESET);
    std::process::exit(255);
}

/// Debugging dump diffrent color text for simple debugging 
pub fn dump(text: &str) {
    println!("{}{}{}DUMPED: {}! {}", COLOR_BOLD, COLOR_ITAL, COLOR_YELLOW, text, COLOR_RESET);
    std::process::exit(69);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_text() {
        output("BLACK", "Hello Wrld");
        output("RED", "Hello Wrld");
        output("GREEN", "Hello Wrld");
        output("YELLOW", "Hello Wrld");
        output("BLUE", "Hello Wrld");
    }
}