pub mod stdin;
pub use crate::supply::stdin::StdinInput;

pub trait LineSupplier {
    fn get_line(&mut self) -> Option<&String>;
}

pub fn process<T: LineSupplier>(supplier: &mut T) {
    loop {
        let line = supplier.get_line();
        match line {
            Some(line) => println!("{}", line),
            None => break
        }
    }
}
