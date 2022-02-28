use std::io;
use std::io::prelude::*;
use std::str;

struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner {
            reader: reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }

    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let t: usize = scan.token();

    for _ in 0..t {
        let n: usize = scan.token();
        let mut even: Vec<usize> = Vec::new();
        let mut odd: Vec<usize> = Vec::new();

        for _ in 0..n {
            let e: usize = scan.token();

            if e % 2 == 0 {
                even.push(e);
            } else {
                odd.push(e);
            }
        }

        if even.is_empty() || odd.is_empty() {
            writeln!(w, "-1").ok();
        } else {
            for (_, v) in even.iter().enumerate() {
                write!(w, "{} ", v).ok();
            }

            for (_, v) in odd.iter().enumerate() {
                write!(w, "{} ", v).ok();
            }

            writeln!(w, "").ok();
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    solve(&mut scan, &mut out);
}
