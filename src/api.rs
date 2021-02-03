#![allow(warnings)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use io::BufReader;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct FlareContext;

impl FlareContext {
    fn textFile(&self, filename: String) -> TextFileRDD {
        TextFileRDD::new(filename)
    }
}

trait RDDBase {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn map<F, U>(self, mapfn: F) -> MapRDD<F, Self>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
    {
        MapRDD {
            mapfn,
            source: self,
        }
    }
}

struct MapRDD<F, R> {
    mapfn: F,
    source: R,
}

impl<F, R, U> RDDBase for MapRDD<F, R>
where
    R: RDDBase,
    F: FnMut(R::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<U> {
        unimplemented!()
    }
}

struct TextFileRDD {
    filename: String,
    iter: io::Lines<io::BufReader<File>>,
}

impl RDDBase for TextFileRDD {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.iter.next() {
            e.ok()
        } else {
            None
        }
    }
}

impl TextFileRDD {
    fn new(filename: String) -> TextFileRDD {
        let iter = read_lines(&filename).unwrap();
        TextFileRDD { filename, iter }
    }
}

#[test]
fn test() {
    let fc = FlareContext {};

    let mut rdd = fc
        .textFile("/Users/adarshrp/.bashrc".to_owned())
        .map(|e| e.len());

    while let Some(line) = rdd.next() {
        println!("-- {:?}", line);
    }
}

fn foo(a: i32, b: i32) {}
