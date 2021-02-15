use std::collections::HashMap;

use scanner::{read_stdin, Scanner};

fn main() {
    let all_stdin = read_stdin();
    let mut scanner = Scanner::new(&all_stdin);

    let q: usize = scanner.next();
    let mut map = HashMap::new();
    for _ in 0..q {
        let t: u8 = scanner.next();
        match t {
            0 => {
                let k: u64 = scanner.next();
                let v: u64 = scanner.next();
                map.insert(k, v);
            }
            1 => {
                let k: u64 = scanner.next();
                let v = map.get(&k).unwrap_or(&0);
                println!("{}", v);
            }
            _ => unreachable!(),
        }
    }
}

mod scanner {
    use std::str::{FromStr, SplitWhitespace};

    pub struct Scanner<'a> {
        iter: SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn next<T>(&mut self) -> T
        where
            T: FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
        {
            self.iter.next().unwrap().parse().unwrap()
        }

        pub fn next_vec<T>(&mut self, n: usize) -> Vec<T>
        where
            T: FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
        {
            let mut v = vec![];
            for _ in 0..n {
                v.push(self.next());
            }
            v
        }

        pub fn chars(&mut self) -> Vec<char> {
            let s: &str = self.iter.next().unwrap();
            s.chars().collect()
        }
    }

    pub fn read_stdin() -> String {
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }
}
