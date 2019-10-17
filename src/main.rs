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
    let mut pos: usize = 0;                  // Position in code
    let mut ptr: usize = 0;                   // Current pointer position
    let mut stack: Vec<usize> = Vec::new();   // Stack (used for [ and ])
    let mut memory: Vec<i64> = vec![0, 64]; // Program memory

    // Just loop over each character and match it to an action
    loop {
        if pos >= program.len() {
            break;
        }

        match program.as_bytes()[pos] as char {
            '>' => {
                ptr += 1;
                if ptr > memory.len() {
                    memory.push(0);
                }
            },
            '<' => ptr -= 1,
            '+' => memory[ptr] += 1,
            '-' => memory[ptr] -= 1,
            '[' => {
                // Search for the matching ] command if byte in memory is zero.
                // This uses the stack to identify the matching bracket.
                stack.push(pos);
                if memory[ptr] == 0 {
                    let old_pos: usize = pos;
                    while stack[stack.len()] >= old_pos {
                        pos += 1;
                        match program.as_bytes()[pos] as char{
                            '[' => stack.push(pos),
                            ']' => {stack.pop();}
                            _ => {}
                        }
                    }
                    pos += 1;
                    continue;
                }
            },
            ']' => {
                // Go back to the most recent [ operator
                if memory[ptr] != 0 {
                    pos = match memory.pop() { Some(x) => x as usize, None => 0};
                }
            }
            '.' => print!("{}", memory[ptr]),
            ',' => {std::io::stdin().bytes().next().and_then(|result|result.ok()).map(|byte| byte as char);}
            _ => {}
        }
        pos += 1;
    }

    exit(0);
}
