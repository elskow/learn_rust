##![allow(unused_imports)]
##![allow(unused_variables)]
##![allow(unused_must_use)]

use std::{
    collections::VecDeque,
    fmt,
    io::{self, BufRead, BufReader, Read, Stdin},
    str::FromStr,
}

struct scanner{
    tokens: VecDeque<String>,
}

impl scanner{
    pub fn new() -> self{
        let stdin =  io::stdin();
        let mut tokens = VecDeque::new();
        for line in stdin.lock().lines(){
            for token in line.unwrap().split_whitespace(){
                tokens.push_back(token.to_owned());
            }
        }
        self {tokens}
    }

    pub fn next<T : fromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: fmt::Debug,
    {
        T::from_str(&self.tokens.pop_front().unwrap().).unwrap()
    }
}