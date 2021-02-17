use std::io::{stdout, BufWriter, Write};

use scanner::{read_stdin, Scanner};

fn main() {
    let s = read_stdin();
    let mut sc = Scanner::new(&s);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let t: usize = sc.next();
    for _ in 0..t {
        let a: u64 = sc.next();
        let b: u64 = sc.next();
        writeln!(out, "{}", a + b).unwrap();
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
