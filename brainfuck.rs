use console::Term;
use std::env;
use std::fs;
use std::process::exit;
use regex::Regex;

const RED:   &str = "\u{001b}[31m";
const GREEN: &str = "\u{001b}[32m";
const CYAN:  &str = "\u{001b}[38;2;145;231;255m";
const WHITE: &str = "\u{001b}[37m";

fn throw_exception(error_name: &str, error_message: &str) -> () {
    println!("{}{}: {}{}", RED, error_name, error_message, WHITE);
    exit(0)
}

fn throw_exception_with_pos(error_name: &str, error_position: i32, error_message: &str) -> () {
    println!("{}{}: at position {} - {}{}", RED, error_name, error_position, error_message, WHITE);
    exit(0)
}

fn display_help() {
    println!("
Brainfuck Interpreter
---------------------

This is an executable that can run brainfuck files, created by axololly on GitHub.
        
To use this, navigate to the directory with the brainfuck file (marked with the .bf
extension) and run a command that looks like this in terminal:

    {0}brainfuck your-file.bf{1}


Extra Details
-------------

- Brainfuck files can be documented with Rust-style comments that get stripped out before execution.
  Any special characters that are usually used in brainfuck execution, when commented, are ignored.

- Whitespace is also stripped before execution, meaning whitespace is {2}irrelevant{1} to code execution.

- The memory is {0}30,000 blocks{1}, and each block {2}cannot{1} exceed the inclusive range of {0}0-255{1}.

- All 8 instructions are the same.\n", CYAN, WHITE, RED);
    exit(0)
}

fn sanitise_code(code: &mut str) -> String {
    let binding = Regex::new(r"\/\/.+")
        .unwrap()
        .replace(code, "");

    let new_code = binding.as_ref();
    
    let binding = Regex::new(r"\n|\r| |\t")
        .unwrap()
        .replace_all(new_code, "");

    let new_code_2 = binding.as_ref();
    
    let binding = Regex::new(r"(\/\*)|(\*\/)")
        .unwrap()
        .replace_all(new_code_2, "");
    
    let new_code_3 = binding.as_ref();
    
    let stray_comment_pos = new_code_3.find("/*");
    
    if let Some(stray_comment_pos) = stray_comment_pos {
        throw_exception_with_pos("SyntaxError", stray_comment_pos as i32, "cannot import code with unterminated multi-line comments. (\"/*\" was found in the code.)");
    }

    let stray_comment_pos = new_code_3.find("*/");

    if let Some(stray_comment_pos) = stray_comment_pos {
        throw_exception_with_pos("SyntaxError", stray_comment_pos as i32, "cannot import code with stray comment characters. (\"*/\" was found in the code.)");
    }

    let while_loop_starts = new_code_3
        .bytes()
        .filter(|c| *c == b'[')
        .count();

    let while_loop_ends = new_code_3
        .bytes()
        .filter(|c| *c == b']')
        .count();

    if while_loop_starts > while_loop_ends {
        let i = code.find("[");

        if let Some(i) = i {
            throw_exception_with_pos("SyntaxError", i as i32, "cannot import code with unterminated while loops. (Unmatched \"[\" was found in the code.)");
        }
    }

    if while_loop_starts < while_loop_ends {
        let i = code.rfind("]");

        if let Some(i) = i {
            throw_exception_with_pos("SyntaxError", i as i32, "cannot import code with trailing while loop characters. (Unmatched \"]\" was found in the code.)");
        }
    }

    new_code_3.to_string()
}

fn execute_code(code: &mut str, show_memory_after: bool) -> () {
    println!("");
    
    let brainfuck_code = sanitise_code(code);

    let mut code_index: usize = 0;
    let mut while_loop_start_indexes: Vec<i32> = Vec::new();

    let mut has_console_output = false;

    let mut memory: [i32; 30_000] = [0; 30_000];
    let mut ptr = 0;
    let mut furthest_ptr = 0;

    while code_index < brainfuck_code.len() {
        let current = brainfuck_code.as_bytes()[code_index] as char;

        match current {
            '>' => {
                // Gone out of rightward bounds
                if ptr == 29_999 {
                    throw_exception("OutOfBoundsError", "cannot move pointer outside of rightward bounds.");
                }

                ptr += 1;
                
                // Keep record of furthest pointer for
                // when we print the memory cells.
                if ptr > furthest_ptr {
                    furthest_ptr = ptr;
                }
            }

            '<' => {
                // Gone out of leftward bounds
                if ptr == 0 {
                    throw_exception("OutOfBoundsError", "cannot move pointer outside of leftward bounds.");
                }

                ptr -= 1;
            }

            '+' => {
                if memory[ptr] == 255 {
                    throw_exception("OverflowError", "cannot increment memory block past integer limit of 255.");
                }

                memory[ptr] += 1;
            }

            '-' => {
                if memory[ptr] == 0 {
                    throw_exception("SubZeroError", "cannot decrement memory block below 0.");
                }

                memory[ptr] -= 1;
            }

            '[' => {
                while_loop_start_indexes.push(code_index as i32);
            }

            ']' => {
                // Send the code pointer back to the start of the while loop
                // if the cell the pointer lands on is above 0.
                if memory[ptr] > 0 {
                    code_index = *while_loop_start_indexes
                        .last()
                        .expect(&*format!("{}Fatal Error: the while loop last indexes array did not contain any indexes.{}", RED, WHITE)) as usize;
                }

                // Otherwise, remove the latest index as we have gone up a
                // level, in terms of nested while loops.
                else {
                    while_loop_start_indexes.pop();
                }
            }

            '.' => {
                println!("{}", char::from_u32(memory[ptr] as u32).unwrap());

                has_console_output = true;
            }

            ',' => {
                let input_char = Term::stdout().read_char().unwrap();

                if input_char as i32 > 255 {
                    throw_exception_with_pos("OverflowError", code_index as i32, "inputted character exceeds value of 255.");
                }

                memory[ptr] = input_char as i32;
            }

            _ => {
                throw_exception_with_pos("SyntaxError", code_index as i32, &*format!("unrecognised character '{}' found in code.", brainfuck_code.as_bytes()[code_index] as char));
            }
        }

        code_index += 1;
    }

    if !has_console_output {
        println!("{}No output provided.{}", RED, WHITE);
    }

    println!();

    if show_memory_after {
        let mut locations_to_values = String::new();

        for i in 0..(furthest_ptr + 1) {
            if memory[i] > 0 {
                let mem_block_repr = &*memory[i].to_string();
                let mem_block_pos = &*i.to_string();
                
                locations_to_values.push_str(
                    &*format!(
                        "{}{: >7}{} - {}[{}]{}",
                        CYAN,
                        mem_block_pos,
                        WHITE,
                        GREEN,
                        mem_block_repr,
                        WHITE
                    )
                );
                locations_to_values.push_str("\n");
            }
        }

        locations_to_values.push_str(&*format!("\n    ptr => {1}{2}{0}", WHITE, CYAN, ptr));

        println!("\n Memory Breakdown\n------------------\n{}", locations_to_values);
    }
}

fn main() {
    // Note that args contains the .exe name, so
    // each of the key arguments is 1-indexed 
    // instead of 0-indexed.
    let args: Vec<String> = env::args().collect();

    // Run the .exe with no arguments
    if args.len() == 1 {
        display_help();
    }

    // Run the .exe with the help argument
    if args[1] == "-h" || args[1] == "--help" {
        display_help();
    }

    /*
    The valid arguments (with debug flag) would be:
    
       brainfuck.exe path-to-file.bf --debug
    
    Which is 3 total arguments.
    */
    if args.len() > 3 {
        throw_exception("ArgumentError", &*format!("too many arguments were provided.\n\n{}If this is meant to be a file path, wrap it in \"quotation marks\"", CYAN));
    }

    let mut show_memory_output = false;

    if args.len() == 3 {
        if args[2] == "-d" || args[2] == "--debug" {
            show_memory_output = true;
        }
        else {
            throw_exception("ArgumentError", &*format!("expected '-d' or '--debug' - received \"{}\".", args[2]));
        }
    }
    
    let file_path = &args[1];

    // If file is not a brainfuck file
    if !file_path.ends_with(".bf") {
        throw_exception("FileLoadError", &*format!("cannot run code from a file that does not have the extension {}.bf", CYAN));
    }

    let mut brainfuck_code = fs::read_to_string(file_path).unwrap();

    // If there's no code to execute
    if brainfuck_code.len() == 0 {
        throw_exception("FileLoadError", "file does not contain any code to execute.");
    }

    execute_code(&mut brainfuck_code, show_memory_output);
}
