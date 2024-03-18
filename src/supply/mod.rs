use std::io::{stdin, BufRead};

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

pub struct StdinInput {
    lines: Vec<String>,
    index: usize
}

impl StdinInput {
    pub fn new() -> Self {
        Self { lines: vec![], index: 0 }
    }
    fn load(&mut self) {
        self.lines = stdin().lock().lines()
            .map(|line| line.unwrap())
            .collect();
    }
}

impl LineSupplier for StdinInput {
    fn get_line(&mut self) -> Option<&String> {
        if self.index == 0 {
            self.load();
        }
        self.index += 1;
        self.lines.get(self.index - 1)
    }
}