pub mod transform {
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
}
