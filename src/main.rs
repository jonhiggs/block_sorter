// #[macro_use]
extern crate indoc;
extern crate regex;

use std::process;
use std::io::{self, Read};

use regex::Regex;


struct Chunk {
    key: String,
    data: String,
}

impl Chunk {
    fn new(k: String, d: String) -> Chunk {
      Chunk { key: k, data: d }
    }

    fn get_key(&self) -> &str {
        &self.key
    }

    fn set_key(&mut self, data: &str) {
        self.key = String::from(data);
    }

    fn push_data(&mut self, data: &str) {
        self.key.push_str(data);
    }
}

fn chunkify(raw: &str) -> Vec<Chunk> {
    let delimeter = Regex::new(r"^    .+").unwrap();
    let mut chunks: Vec<Chunk> = vec![];

    let mut chunk = Chunk::new(String::from(""), String::from(""));

    for line in raw.lines() {
        let l = String::from(line);

        if !delimeter.is_match(&l) {
            chunk.set_key(&l);
        } else {
            chunk.push_data("\n");
            chunk.push_data(&l);
            continue
        }
        chunks.push(chunk);
    }

    chunks
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);

    chunkify(&buffer);

    process::exit(0);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunkify() {
        let raw_data = String::from("line one\nline two");
        //assert_eq!(chunkify(&raw_data).len(), 2);
        assert_eq!(chunkify(&raw_data).first().unwrap().get_key(), "line one");
    }

    #[test]
    fn test_chunk() {
        {
            let chunk = Chunk::new(String::from("the_key"), String::from("the_data"));
            assert_eq!(chunk.get_key(), "the_key");
        }
    }

    //        assert_eq!(chunk.data(), "line one\nline two");
    //        assert_eq!(chunk.children().len(), 0);
    //    }
    //    {
    //        let chunk = Chunk {
    //            raw_data: String::from("line one\n    line two")
    //        };

    //        //assert_eq!(chunk.data(), "line one\n    line two");
    //        assert_eq!(chunk.children().len(), 1);
    //        //assert_eq!(chunk.children().first().unwrap().data(), "    line two");
    //    }
    //}
}
