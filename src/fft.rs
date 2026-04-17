// fft.rs — Algoritmo de Cooley-Tukey (radix-2 DIT)

use std::f64::consts::PI;

/// Número complejo mínimo para la FFT
#[derive(Clone, Copy, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Multiplicación de complejos
    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

/// Reordena las muestras con bit-reversal (necesario en Cooley-Tukey)
fn bit_reverse_copy(x: &[Complex]) -> Vec<Complex> {
    let n = x.len();
    let bits = (n as f64).log2() as usize;
    let mut out = x.to_vec();

    for i in 0..n {
        let rev = (0..bits)
            .fold(0usize, |acc, b| acc | (((i >> b) & 1) << (bits - 1 - b)));
        if rev > i { out.swap(i, rev); }
    }
    out
}

/// FFT iterativa de Cooley-Tukey — O(N log N)
pub fn fft_cooley_tukey(x: &[f64]) -> Vec<Complex> {
    let n = x.len();
    assert!(n.is_power_of_two(), "N debe ser potencia de 2");

    let mut a: Vec<Complex> = x.iter()
        .map(|&v| Complex::new(v, 0.0))
        .collect();
    a = bit_reverse_copy(&a);

    let mut len = 2;
    while len <= n {
        // Factor de torsión (twiddle factor) base
        let ang = -2.0 * PI / len as f64;
        let wlen = Complex::new(ang.cos(), ang.sin());

        for i in (0..n).step_by(len) {
            let mut w = Complex::new(1.0, 0.0);
            for j in 0..(len / 2) {
                let u = a[i + j];
                let v = w.mul(a[i + j + len / 2]);
                // Mariposa (butterfly)
                a[i + j]           = Complex::new(u.re + v.re, u.im + v.im);
                a[i + j + len / 2] = Complex::new(u.re - v.re, u.im - v.im);
                w = w.mul(wlen);
            }
        }
        len *= 2;
    }
    a
}