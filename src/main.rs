// main.rs — Analizador espectral de señales

mod fft;
mod signal;

use fft::fft_cooley_tukey;
use signal::generate_signal;

fn main() {
    // Parámetros de muestreo
    let sample_rate: f64 = 1000.0;   // Hz
    let n_samples:   usize = 512;      // debe ser potencia de 2

    // Señal compuesta: 50 Hz + 120 Hz + 200 Hz
    let freqs      = [50.0, 120.0, 200.0];
    let amplitudes = [1.0, 0.6, 0.3];

    let samples = generate_signal(
        &freqs, &amplitudes, sample_rate, n_samples
    );

    println!("=== Analizador FFT ===\nFs = {} Hz | N = {} muestras\n",
        sample_rate, n_samples);
    
    // Calcular FFT
    let spectrum = fft_cooley_tukey(&samples);

    // Espectro de un solo lado (bins positivos)
    let half = n_samples / 2;
    let freq_res = sample_rate / n_samples as f64;

    let power: Vec<f64> = spectrum[..half]
        .iter()
        .map(|c| (c.re * c.re + c.im * c.im).sqrt() * 2.0 / n_samples as f64)
        .collect();

    // Frecuencia dominante
    let (peak_bin, peak_mag) = power
        .iter()
        .enumerate()
        .skip(1)
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    let dominant_hz = peak_bin as f64 * freq_res;
    println!("Frecuencia dominante: {:.1} Hz  (mag = {:.4})\n",
        dominant_hz, peak_mag);

    // Imprimir espectro (bins con mayor energía)
    println!("{:>8}  {:>8}  {}", "Frec(Hz)", "Magnitud", "Barra");
    println!("{}", "-".repeat(50));

    let threshold = 0.05;
    for (i, &mag) in power.iter().enumerate() {
        if mag >= threshold {
            let freq = i as f64 * freq_res;
            let bar_len = (mag * 40.0) as usize;
            let bar = "█".repeat(bar_len);
            println!("{:8.1}  {:8.4}  {}", freq, mag, bar);
        }
    }
}