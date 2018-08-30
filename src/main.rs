pub mod vm;
pub mod instruction;
pub mod repl;
pub mod compiler;

fn main() {
    println!("Initialising....");

    let mut repl = repl::REPL::new();

    repl.run();
}
