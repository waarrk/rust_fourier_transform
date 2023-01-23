use std::f64::consts::PI;

mod fourier;

fn main() {
    let flame_length = 1000;
    let sampling_freq = 16000;
    let hz = 2000;

    let sin_curve: Vec<f64> = (0..flame_length)
        .map(|i| (i as f64) * 2.0 * PI * (hz as f64) / (sampling_freq as f64))
        .map(|a| a.sin())
        .collect();

    let spectrum = fourier::transform::dft(sin_curve, sampling_freq);

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

    print!("Hello");
}
