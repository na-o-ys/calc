extern crate rand;

use std::ops::{Index, IndexMut};
use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

fn eliminate(m: &mut Mat) {
    let n = m.size();
    for i in 0..n {
        let pivot = m[i][i];
        for j in i..(n + 1) {
            m[i][j] = (1.0 / pivot) * m[i][j];
        }
        m[i][i] = 1.0;
        for k in (i + 1)..n {
            let c = m[k][i];
            for l in i..(n + 1) {
                m[k][l] -= c * m[i][l];
            }
        }
    }
    for i in (1..n).rev() {
        for j in (0..i).rev() {
            m[j][n] -= m[j][i] * m[i][n];
            m[j][i] = 0.0;
        }
    }
}

#[derive(Debug)]
struct Mat {
    v: Vec<Vec<f32>>
}

impl Mat {
    fn new(v: Vec<Vec<f32>>) -> Mat {
        Mat { v: v }
    }

    fn size(&self) -> usize {
        self.v.len()
    }
}

impl Index<usize> for Mat {
    type Output = Vec<f32>;
    fn index(&self, index: usize) -> &Vec<f32> {
        &self.v[index]
    }
}

impl IndexMut<usize> for Mat {
    fn index_mut<'a>(&'a mut self, index: usize) -> &mut Vec<f32> {
        &mut self.v[index]
    }
}

fn main() {
    let n = read_size();
    let mut rng = rand::thread_rng();

    let mut rows = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::with_capacity(n + 1);
        for _ in 0..(n + 1) {
            row.push(rng.next_f32());
        }
        rows.push(row);
    }
    let mut m = Mat::new(rows);

    let start = Instant::now();
    eliminate(&mut m);
    println!("{} s", start.elapsed().as_secs());

    if n < 100 {
        let mut ans = Vec::with_capacity(n);
        for i in 0..m.size() {
            ans.push(m[i][m.size()]);
        }
        println!("{:?}", ans);
    }
}

fn read_size() -> usize {
    read_line().parse().unwrap()
}
 
fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}
