use std;
use std::io;
use std::io::Write;
use std::io::Read;

use std::fs::File;
use std::path::Path;

use vm::VM;

use compiler::Scanner;
use compiler::token::Token;
use compiler::parser::Parser;
use compiler::parser::ParseResult;

pub struct REPL {

    command_buffer: Vec<String>,

    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the i_v REPL loop");

        let stdin = io::stdin();

        loop {

            let mut buffer = String::new();

            print!(">>> ");
            io::stdout().flush()
                .expect("Unable to flush stdout");

            stdin.read_line(&mut buffer)
                .expect("Unable to read input");

            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                },

                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },

                ".clear_registers" => {

                    println!("Clearing registers...");

                    for i in 0..self.vm.registers.len() {
                        self.vm.registers[i] = 0;
                    }
                },

                ".list_registers" => {

                    println!("Listing registers...");

                    println!("{:#?}", self.vm.registers);
                },

                ".cleanup" => {

                    println!("Clearing program...");

                    self.vm.program.truncate(0);

                    for i in 0..self.vm.registers.len() {
                        self.vm.registers[i] = 0;
                    }
                },

                ".program" => {

                    println!("Listing current instructions in program...");

                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                },

                ".help" => {
                    println!("Current commands: ");
                    println!("> .help");
                    println!("> .history");
                    println!("> .cleanup");
                    println!("> .clear_registers");
                    println!("> .list_registers");
                    println!("> .program");
                    println!("> .quit");
                },

                ".load" => {
                    println!("Please enter the file you wish to load");
                    print!("> ");
                    io::stdout().flush().expect("Unable to flush output");

                    let mut tmp = String::new();

                    stdin.read_line(&mut tmp).expect("Unable to read input");
                    let tmp = tmp.trim();

                    let file_name = Path::new(&tmp);
                    let mut f = File::open(file_name).expect("Unable to open file");

                    let mut contents = String::new();
                    f.read_to_string(&mut contents).expect("Unable to read file");

                    let mut scanner = Scanner::new(&contents);

                    let mut tokens = vec!();

                    loop {
                        let tok = scanner.next_token();
                        tokens.push(tok.clone());

                        println!("{:?}", tok);

                        if tok == Token::EOF {
                            break;
                        }
                    }

                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        ParseResult::Success(expr) => {
                            println!("we did it?")
                        },
                        _ => println!("Something went wrong")
                    }
                },

                _ => {
                    let mut scanner = Scanner::new(&buffer);

                    let mut tokens = vec!();

                    loop {
                        let tok = scanner.next_token();
                        tokens.push(tok.clone());

                        println!("{:?}", tok);

                        if tok == Token::EOF {
                            break;
                        }
                    }

                    tokens.reverse();

                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        ParseResult::Success(expr) => {
                            println!("we did it? {:?}", expr);
                        },
                        ParseResult::Failed(str) => {
                            println!("Something went wrong");
                            println!("{}", str);
                        }
                    }
                }
            }
        }
    }
}
