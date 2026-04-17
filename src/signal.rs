// signal.rs — Generador de señales de prueba

use std::f64::consts::PI;

/// Genera una señal multi-tono: suma de senos con frecuencias y amplitudes dadas
///
/// # Argumentos
/// * `freqs`       — frecuencias de cada componente [Hz]
/// * `amplitudes`  — amplitudes correspondientes
/// * `sample_rate` — frecuencia de muestreo [Hz]
/// * `n`           — número de muestras (potencia de 2)
pub fn generate_signal(
    freqs:       &[f64],
    amplitudes:  &[f64],
    sample_rate: f64,
    n:           usize,
) -> Vec<f64> {
    (0..n).map(|i| {
        let t = i as f64 / sample_rate;
        freqs.iter()
             .zip(amplitudes.iter())
             .map(|(&f, &a)| a * (2.0 * PI * f * t).sin())
             .sum()
    }).collect()
}

// Aplica ventana de Hann para reducir spectral leakage
// pub fn apply_hann_window(signal: &mut Vec<f64>) {
//     let n = signal.len();
//     for (i, s) in signal.iter_mut().enumerate() {
//         let w = 0.5 * (1.0 - (2.0 * PI * i as f64
//                             / (n - 1) as f64).cos());
//         *s *= w;
//     }
// }