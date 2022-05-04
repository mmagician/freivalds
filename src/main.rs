use ark_ff::{fields::Fp64, MontBackend, MontConfig, MontFp};
use ark_ff::{BigInt, PrimeField};
use ark_ff::{BigInteger64, UniformRand};
use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ark_std::str::FromStr;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, Error, ErrorKind};
use std::ops::MulAssign;
use std::ops::{Deref, Mul};
use std::path::Path;
use std::{fs, io};

#[derive(MontConfig)]
#[modulus = "9739"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

type Vector = Vec<Fq>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short)]
    a_filename: String,

    #[clap(short)]
    b_filename: String,

    #[clap(short)]
    c_filename: String,
}

struct Matrix(Vec<Vector>);
/// impl Deref so we can access the anonymous
/// field directly by calling an instance of the matrix
impl Deref for Matrix {
    type Target = Vec<Vector>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let mut result = Vector::new();
        for row in self.iter() {
            // for entry in row.iter() {
            //     // println!("{:?}", entry);
            // }
            let value = row
                .iter()
                .zip(rhs)
                .fold(MontFp!(Fq, "0"), |acc, (x, y)| acc + (*x * y));
            result.push(value);
        }

        println!("{:?}", result);
        // self.into_iter()
        //     .for_each(|x| x.into_iter().zip(rhs).map(|(a, b)| a * b));
        result
    }
}

fn main() {
    let args = Args::parse();
    let paths = vec![args.a_filename, args.b_filename, args.c_filename];
    let matrices: Vec<Matrix> = paths
        .into_iter()
        .map(|path| read_matrix(&path).unwrap())
        .collect();

    let accept = verify(&matrices[0], &matrices[1], &matrices[2]);
}

fn read_matrix(filename: &str) -> Result<Matrix, ()> {
    if let Ok(lines) = read_lines(filename) {
        let mut rows: Vec<Vector> = Vec::new();
        for line in lines {
            if let Ok(row) = line {
                let row_string: Vector = row
                    .split(",")
                    .map(|elem: &str| Fq::from_str(elem).unwrap())
                    .collect();
                rows.push(row_string);
            }
        }
        Ok(Matrix(rows))
    } else {
        Err(())
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn verify(a: &Matrix, b: &Matrix, c: &Matrix) -> bool {
    // 1. pick a random r from the prime field
    let mut rng = ark_std::test_rng();
    let r: Fq = rng.gen();

    // 2. Compute the vector x = r, r^2, ...r^n
    let mut x_vec: Vector = Vec::with_capacity(a.len());
    let mut temp: Fq = MontFp!(Fq, "1");
    for _ in 1..=a.len() {
        temp *= r;
        x_vec.push(temp);
    }
    // println!("Vec x is {:?}", x_vec);

    // 3. Multiply w=B.x
    let w: Vector = b * &x_vec;
    // 4. Multiply A.w
    let y: Vector = a * &w;
    // 5. Compute C.x
    let z: Vector = c * &x_vec;
    // 6. Verify C.x == A.w
    y == z
}

#[test]
fn test_serialization() {
    let paths = fs::read_dir("./src/testdata").unwrap();
    for path in paths {
        assert!(read_matrix(path.unwrap().path().to_str().unwrap()).is_ok());
    }
}
