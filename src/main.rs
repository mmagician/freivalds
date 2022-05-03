use ark_ff::UniformRand;
use ark_ff::{fields::Fp64, MontBackend, MontConfig, MontFp};
use ark_ff::{BigInt, PrimeField};
use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ark_std::str::FromStr;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, Error, ErrorKind};
use std::ops::Deref;
use std::ops::MulAssign;
use std::path::Path;
use std::{fs, io};

#[derive(MontConfig)]
#[modulus = "17"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short)]
    a_filename: String,

    #[clap(short)]
    b_filename: String,

    #[clap(short)]
    c_filename: String,

    /// Prime field modulus
    #[clap(short, long, default_value_t = 7)]
    modulus: u64,
}

struct Matrix(Vec<Vec<Fq>>);
/// impl Deref so we can access the anonymous
/// field directly by calling an instance of the matrix
impl Deref for Matrix {
    type Target = Vec<Vec<Fq>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let args = Args::parse();
    let paths = vec![args.a_filename, args.b_filename, args.c_filename];
    let matrices: Vec<Matrix> = paths
        .into_iter()
        .map(|path| read_matrix(&path).unwrap())
        .collect();

    let modulus = args.modulus;
    // impl MontConfig<1>
    // struct A();
    let accept = verify(&matrices[0], &matrices[1], &matrices[2], modulus);
}

fn read_matrix(filename: &str) -> Result<Matrix, ()> {
    if let Ok(lines) = read_lines(filename) {
        let mut rows: Vec<Vec<Fq>> = Vec::new();
        for line in lines {
            if let Ok(row) = line {
                let row_string: Vec<Fq> = row
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

fn verify(a: &Matrix, b: &Matrix, c: &Matrix, p: u64) -> bool {
    // 1. pick a random r from the prime field
    let mut rng = ark_std::test_rng();
    let r: Fq = rng.gen();

    // 2. Compute the vector x = r, r^2, ...r^n
    let mut x_vec: Vec<Fq> = Vec::with_capacity(a.len());
    let mut temp: Fq = MontFp!(Fq, "1");
    for _ in 1..=a.len() {
        temp *= r;
        x_vec.push(temp);
    }
    println!("Vec x is {:?}", x_vec);

    // 3. Multiply w=B.x
    // 4. Multiply A.w
    // 5. Compute C.x
    // 6. Verify C.x == A.w

    true
}

#[test]
fn test_serialization() {
    let paths = fs::read_dir("./src/testdata").unwrap();
    for path in paths {
        assert!(read_matrix(path.unwrap().path().to_str().unwrap()).is_ok());
    }
}
