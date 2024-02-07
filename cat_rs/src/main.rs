use std::env;
use std::fs::File;
use std::io::Read;

fn help() {
    let help_str: &'static str = "
Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help        display this help and exit
      --version     output version information and exit

Examples:
  cat f - g  Output f's contents, then standard input, then g's contents.
  cat        Copy standard input to standard output.

GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
Full documentation <https://www.gnu.org/software/coreutils/cat>
or available locally via: info '(coreutils) cat invocation'";
    println!("{}", help_str);
}

fn add_line_numbers(contents: &mut String) -> String {
    contents
        .lines()
        .enumerate()
        .map(|(i, line)| format!("  {:4} {}", i+1, line))
        .collect::<Vec<_>>()
        .join("\n")
}


fn add_end_chars(contents: &mut String, end_char: char) -> String {
    contents
        .lines()
        .enumerate()
        .map(|(_i, line)| format!("{}{}", line, end_char))
        .collect::<Vec<_>>()
        .join("\n")
}

fn add_tab_chars(contents: &mut String) -> String {
    contents.replace("  ", "^I")
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let arg_1 = &args[1];
    match arg_1.as_str() {
        "--help" => {
            help();
            return Ok(());
        },
        "-n" | "--number" => {
            if args.len() < 3 {
                println!("Missing filename!");
                return Ok(());
            }
            let filename = &args[2];
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let line_numbered_content = add_line_numbers(&mut contents);
            println!("{}", line_numbered_content);
        },
        "-E" | "--show-ends" => {
            if args.len() < 3 {
                println!("Missing filename!");
                return Ok(());
            }
            let filename = &args[2];
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let line_delimited_content = add_end_chars(&mut contents, '$');
            println!("{}", line_delimited_content);
        },
        "-T" | "--show-tabs" => {
            if args.len() < 3 {
                println!("Missing filename!");
                return Ok(());
            }
            let filename = &args[2];
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let tab_replaced_content = add_tab_chars(&mut contents);
            println!("{}", tab_replaced_content);
        }
        _ =>  {
            let filename = &args[1];
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("{}", contents);
        }
    }

    Ok(())
}