use cmd::args;

mod cmd;
mod interface;
mod protocol;

fn main() { 
    println!("{:#?}", args());
}
