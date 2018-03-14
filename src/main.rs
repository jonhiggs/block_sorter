#[macro_use]
extern crate indoc;
extern crate regex;

use std::process;
use std::io::{self, Read};

use regex::Regex;


// fn read_block() -> Vec(str) {
// }

fn has_chunks(d: &str) -> bool {
    Regex::new(r"\n    ")
        .unwrap()
        .is_match(d)
}

fn chunks(d: &str) -> Vec<String> {
    let mut chunks: Vec<String> = vec![];
    let delimeter = Regex::new(r"^    .+").unwrap();

    for line in d.lines() {
        let l = String::from(line);

        if !delimeter.is_match(&l) {
            chunks.push(l);
        } else {
            let i = chunks.len() - 1;
            chunks[i] = chunks[i].to_string() + "\n" + &l.to_string();
        }
    }

    return chunks;
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);

    for c in chunks(&buffer) {
        if has_chunks(&c) {
            println!("{} has chunks", &c);
        } else {
            println!("{}", c );
        }
    }

    process::exit(0);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunks() {
        {
            let data = indoc!("
                line one
                line two
            ");
            assert_eq!(chunks(data), ["line one", "line two"]);
        }
        {
            let data = indoc!("
                line one
                    line two
                line three
            ");
            assert_eq!(chunks(data), ["line one\n    line two", "line three"]);
        }
    }

    #[test]
    fn test_has_chunks() {
        assert_eq!(false, has_chunks("nope"));
        assert_eq!(false, has_chunks("one\ntwo\n"));
        assert_eq!(true,  has_chunks("something\n    somethingelse"));
        assert_eq!(true,  has_chunks("    something\n    somethingelse"));
    }
}
