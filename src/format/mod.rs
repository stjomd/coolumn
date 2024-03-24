use std::borrow::Cow;
use std::cmp::max;
use lazy_static::lazy_static;
use regex::Regex;
use crate::errors::Error;
use crate::supply::{LineSupplier, Progress};

lazy_static! {
    static ref UNPRINTABLE_REGEX: Regex = Regex::new(r"\p{Cc}\[[0-9;]*[mK]").unwrap();
}

fn get_printable_slice(string: &str) -> String {
    return match UNPRINTABLE_REGEX.replace_all(string, "") {
        Cow::Borrowed(str) => str.to_string(),
        Cow::Owned(str) => str,
    }
}

pub fn get_max_length(supplier: &mut Box<dyn LineSupplier>) -> Result<u32, Error> {
    let mut maxx: u32 = 0;
    loop {
        match supplier.get_line()? {
            Progress::Line(line) => {
                maxx = max(maxx, get_printable_slice(line).len() as u32);
            },
            Progress::Continue => continue,
            Progress::Done => return Ok(maxx)
        }
    }
}

fn get_item(supplier: &mut Box<dyn LineSupplier>) -> Result<Option<String>, Error> {
    loop {
        match supplier.get_line()? {
            Progress::Line(line) => return Ok(Some(line.to_string())),
            Progress::Continue => continue,
            Progress::Done => return Ok(None),
        }
    }
}

pub fn print(supplier: &mut Box<dyn LineSupplier>) -> Result<(), Error> {
    let max_length = get_max_length(supplier)?;
    let terminal_columns = termsize::get().unwrap().cols as u32;
    let columns = terminal_columns / (max_length + 1);
    let mut counter = 0;
    supplier.reset();
    loop {
        let item = get_item(supplier)?;
        if let Some(item) = item {
            let trimmed = item.trim_end();
            let printable_len = get_printable_slice(&trimmed).len();
            let pad = (max_length as usize) - printable_len;
            let diff = item.len() - printable_len;
            print!("{: <width$} ", trimmed, width = printable_len + pad + diff);
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