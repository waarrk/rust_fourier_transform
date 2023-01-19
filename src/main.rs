use num::complex::Complex;
use std::f64::consts::PI;

pub fn dft(frames: Vec<f64>, sampling_freq: i64) -> Vec<Complex<f64>> {
    let mut spectrum: Vec<Complex<f64>> = Vec::new();
    for i in 1..=(sampling_freq / 2) {
        let mut sigma = Complex::new(0.0, 0.0);
        for (j, frame) in frames.iter().enumerate() {
            let delta_t = (j as f64) / (sampling_freq as f64);
            let omega = 2.0 * PI * (i as f64);
            let exponent = Complex::new(0.0, -omega * delta_t);
            sigma += frame * exponent.exp();
        }
        spectrum.push(sigma);
    }
    return spectrum;
}

fn main() {
    let flame_length = 1000;
    let sampling_freq = 16000;
    let hz = 2000;

    let sin_curve: Vec<f64> = (0..flame_length)
        .map(|i| (i as f64) * 2.0 * PI * (hz as f64) / (sampling_freq as f64))
        .map(|a| a.sin())
        .collect();

    let spectrum = dft(sin_curve, sampling_freq);

    let (max_index, max) =
        spectrum
            .iter()
            .enumerate()
            .fold((usize::MIN, f64::MIN), |(i_a, a), (i_b, &b)| {
                if b.norm() > a {
                    (i_b, b.norm())
                } else {
                    (i_a, a)
                }
            });
    println!("spectrum len: {:?}", spectrum.len());
    println!("max spectrum f: {:?}", max_index + 1);
    println!("max spectrum: {:?}", max);
}
