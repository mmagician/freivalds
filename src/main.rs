use ark_ff::{fields::Fp64, MontBackend, MontConfig, MontFp};
use ark_ff::{BigInt, PrimeField};
use clap::Parser;
use std::fs;

#[derive(MontConfig)]
#[modulus = "9739"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

/// Simple program to greet a person
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

struct Matrix {}

impl From<Vec<u8>> for Matrix {
    fn from(_: Vec<u8>) -> Self {
        todo!()
    }
}

fn main() {
    let args = Args::parse();
    let a_contents =
        fs::read(&args.a_filename).expect(format!("Error reading {}", args.a_filename).as_str());

    let a: Matrix = Matrix::from(a_contents);

    let modulus = args.modulus;
    // let accept = verify(a, b, c, modulus);
    // println!("The ")
}

fn verify(a: Matrix, b: Matrix, c: Matrix, p: u64) -> bool {
    // 1. pick a random r from the prime field

    // 2. Compute the vector x = r, r^2, ...r^n

    // 3. Multiply w=B.x
    // 4. Multiply A.w
    // 5. Compute C.x
    // 6. Verify C.x == A.w

    true
}
