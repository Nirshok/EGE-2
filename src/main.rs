use ege_2::*;
use std::io::stdin;
fn main() {
    let contents;
    loop {
        println!("Enter the file name:\n(If you want to quit, enter 'break'.)\n");

        let mut buff = String::new();
        match stdin().read_line(&mut buff) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("\nError reading input: {err}\n");
                continue;
            }
        }
        let name = buff.trim();

        if name == "break" {
            return println!("\nShutting down.");
        }

        contents = match read_file(name) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("\nError opening the file: {err}\n");
                continue;
            }
        };
        break;
    };

    let lines = amount_of_lines(contents);
    
    println!("\nThe number of lines satisfying the condition is:\n{lines}")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = String::from(
            "99 55 66 55\r33 22 11 12\r4 5 6 7\r77 5 32 1\r99 98 2 15\r65 77 85 3\r11 12 21 121"
        );

        let lines = amount_of_lines(file);

        assert_eq!(lines, 1);
    }

}