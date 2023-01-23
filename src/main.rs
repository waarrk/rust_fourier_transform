use std::ffi::OsString;
use std::process;

use rust_fourier_transform::file_controller::args;

mod file_controller;
mod fourier;

fn main() {
    let mut args: [OsString; 2] = [OsString::from(""), OsString::from("")];
    let mut matrix: ndarray::Array2<f64> = ndarray::Array2::zeros((0, 0));
    args::get_all()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, arg)| {
            args[i] = arg.clone();
        });

    matrix = match file_controller::file_io::csv_to_ndarray(
        file_controller::file_io::fopen(&args[0]).unwrap(),
    ) {
        Ok(matrix) => matrix,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let sampling_freq = 8000;

    let frames = matrix
        .slice(ndarray::s![.., 1])
        .to_vec()
        .iter()
        .map(|x| *x as f64)
        .collect::<Vec<f64>>();

    let spectrum = fourier::transform::dft(frames, sampling_freq);

    for (i, s) in spectrum.iter().enumerate() {
        let amp = s.norm() / matrix.shape()[0] as f64;
        println!("{}, {}", i, amp);
    }
}
