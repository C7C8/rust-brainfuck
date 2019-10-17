use std::env;
use std::fs;
use std::io::Read;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Need to provide an input brainfuck file");
        exit(1);
    }

    let program: String = fs::read_to_string(&args[1]).expect("Could not read from file!");
    let mut pos: usize = 0;                     // Position in code
    let mut ptr: usize = 0;                     // Current pointer position
    let mut stack: Vec<usize> = Vec::new();     // Stack (used for [ and ])
    let mut memory: Vec<u8> = vec![0];          // Program memory

    // Just loop over each character and match it to an action
    loop {
        if pos >= program.len() {
            break;
        }

        match program.as_bytes()[pos] as char {
            '>' => {
                ptr += 1;
                if ptr > memory.len() - 1 {
                    memory.push(0);
                }
            },
            '<' => {
                if ptr == 0 {
                    eprintln!("Attempted to decrement pointer below 0")
                    exit(2);
                }
                ptr -= 1
            },
            '+' => if memory[ptr] == 255 {memory[ptr] = 0} else {memory[ptr] += 1},
            '-' => if memory[ptr] == 0 {memory[ptr] = 255} else {memory[ptr] -= 1},
            '[' => {
                // Search for the matching ] command if byte in memory is zero.
                // This uses the stack to identify the matching bracket.
                stack.push(pos);
                if memory[ptr] == 0 {
                    let old_pos: usize = pos;
                    while !stack.is_empty() && stack[stack.len() - 1] >= old_pos {
                        pos += 1;
                        match program.as_bytes()[pos] as char{
                            '[' => stack.push(pos),
                            ']' => {stack.pop();}
                            _ => {}
                        }
                    }
                }
            },
            ']' => {
                // Go back to the most recent [ operator
                if memory[ptr] != 0 {
                    pos = stack[stack.len() - 1];
                }
                else {
                    stack.pop();
                }
            }
            '.' => print!("{}", memory[ptr] as char),
            ',' => {std::io::stdin().bytes().next().and_then(|result|result.ok()).map(|byte| byte as char);}
            _ => {}
        }
        pos += 1;
    }

    exit(0);
}
