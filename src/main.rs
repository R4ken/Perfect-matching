mod matrix;
mod imod;
use imod::Imod;
use std::io::{self, BufRead};
use rand::RngExt;

use crate::matrix::ModuloMatrix;

fn read_two_ints<R: BufRead>(reader: &mut R, buffer: &mut String) -> (usize, usize) {
    buffer.clear();
    
    reader.read_line(buffer).expect("Failed to read line");

    let mut iter = buffer.split_ascii_whitespace();

    let a: usize = iter.next().expect("Missing first int").parse().expect("Invalid int");
    let b: usize = iter.next().expect("Missing second int").parse().expect("Invalid int");

    (a, b)    
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); 
    let mut buffer = String::with_capacity(64);
    let mut rng = rand::rng();
    let modulo = 1e9 as i32 + 7;
    let z = Imod::from(0);

    let mut n;
    let m;

    (n, m) = read_two_ints(&mut reader, &mut buffer);
    let mut id: Vec<usize> = (0..n).collect();
    let mut mat = ModuloMatrix::new(n);
    for i in 0..n {
        mat[i][i] = Imod::from(0);
    }
    for _ in 0..m {
        let num = rng.random_range(1..(1e9 as i32 + 7));
        let (mut u, mut v) = read_two_ints(&mut reader, &mut buffer);
        if u < v {
            (u, v) = (v, u);
        }
        mat[u][v] = Imod::from(num);
        mat[v][u] = Imod::from(modulo - num)
    }
    if n % 2 != 0 {
        println!("FALSE");
        return;
    }
    let Some(mut inv) = mat.calculate_inverse() else {
        println!("FALSE");
        return;
    };
    println!("True");
    'reduce_vertices: while n > 0 {
        for i in 0..n {
            for j in i + 1..n {
                if mat[i][j] != z && inv[i][j] != z {
                    let res = ModuloMatrix::reduce_rows_and_columns(&mut mat, &mut inv, i, j);
                    if res.is_ok() {
                        println!("{} {}", id[i], id[j]);
                        for k in j..n - 1 {
                            id.swap(k, k + 1);
                        }
                        for k in i..n - 1 {
                            id.swap(k, k + 1);
                        }
                        n -= 2;
                        continue 'reduce_vertices;
                    }
                }
            }
        }
        panic!("Endless loop")
    }
}
