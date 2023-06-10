use std::{
    collections::VecDeque,
    fmt,
    io::{self, BufRead},
    str::FromStr,
};
 
struct Scanner {
    tokens: VecDeque<String>,
}

impl Scanner {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let mut tokens = VecDeque::new();
        for line in stdin.lock().lines() {
            for token in line.unwrap().split_whitespace() {
                tokens.push_back(token.to_owned());
            }
        }
        Self { tokens }
    }
 
    pub fn next<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: fmt::Debug,
    {
        T::from_str(&self.tokens.pop_front().unwrap()).unwrap()
    }
}

fn getline() -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).unwrap();
    ret
}


fn main(){
    let mut sc = Scanner::new();
    let a: i32 = sc.next();
    let _b: i32 = sc.next();
    println!("{}", a);
}