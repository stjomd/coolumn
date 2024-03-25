use std::cmp::max;
use crate::errors::Error;
use crate::supply::{Line, LineSupplier, Progress};

fn get_max_length(supplier: &mut Box<dyn LineSupplier>) -> Result<usize, Error> {
    let mut max_length: usize = 0;
    loop {
        match supplier.get_line()? {
            Progress::Line(line) => {
                max_length = max(max_length, line.clean_line.len());
            },
            Progress::Continue => continue,
            Progress::Done => return Ok(max_length)
        }
    }
}

pub fn print(supplier: &mut Box<dyn LineSupplier>) -> Result<(), Error> {
    let max_item_length = get_max_length(supplier)?;
    let terminal_columns = termsize::get().unwrap().cols as usize;
    let columns = terminal_columns / (max_item_length + 1);
    // Start reading from the beginning & print items
    supplier.reset();
    let mut counter = 0;
    loop {
        // Get the next item to print
        let item: Option<Line>;
        loop {
            match supplier.get_line()? {
                Progress::Line(line) => { item = Some(line); break },
                Progress::Continue => { continue },
                Progress::Done => { item = None; break },
            }
        };
        // Print with padding
        if let Some(item) = item {
            let width = max_item_length + item.line.len() - item.clean_line.len();
            print!("{: <width$} ", item.line.trim_end(), width = width);
            counter += 1;
            if counter % columns == 0 {
                print!("\n");
            }
        } else {
            if counter % columns != 0 {
                print!("\n");
            }
            break
        }
    }
    Ok(())
}