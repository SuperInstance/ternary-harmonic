//! # ternary-harmonic
//! Harmonic series and overtones for ternary signals.

#![forbid(unsafe_code)]

use std::f64::consts::PI;

/// A ternary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trit {
    Neg,
    Zero,
    Pos,
}

/// Return the fundamental frequency of a ternary pattern.
/// A repeating pattern of length `len` at `sample_rate` Hz has fundamental = sample_rate / len.
pub fn fundamental(pattern_len: usize, sample_rate: f64) -> f64 {
    if pattern_len == 0 { return 0.0; }
    sample_rate / pattern_len as f64
}

/// Generate harmonic series: overtones at n*f for n = 1..=count.
pub fn harmonic_series(base_freq: f64, count: usize) -> Vec<f64> {
    (1..=count).map(|n| base_freq * n as f64).collect()
}

/// Generate subharmonic at 1/n of the base frequency.
pub fn subharmonic(base_freq: f64, n: usize) -> f64 {
    if n == 0 { return 0.0; }
    base_freq / n as f64
}

/// Determine which harmonic index dominates a ternary signal segment.
/// Returns the index n where n*base_freq is closest to `frequency`.
pub fn overtone_index(frequency: f64, base_freq: f64) -> usize {
    if base_freq <= 0.0 { return 0; }
    let n = (frequency / base_freq).round() as usize;
    if n == 0 { 1 } else { n }
}

/// Measure total harmonic distortion: ratio of non-fundamental energy to total.
/// `harmonics` contains amplitudes of harmonics 1..N.
pub fn harmonic_distortion(harmonics: &[f64]) -> f64 {
    if harmonics.is_empty() || harmonics[0] == 0.0 { return 0.0; }
    let fundamental = harmonics[0].powi(2);
    let total: f64 = harmonics.iter().map(|&a| a.powi(2)).sum();
    if total == 0.0 { return 0.0; }
    (total - fundamental) / total
}

/// A chord: multiple harmonics sounding together.
#[derive(Debug, Clone)]
pub struct Chord {
    pub frequencies: Vec<f64>,
    pub amplitudes: Vec<f64>,
}

impl Chord {
    pub fn new(frequencies: Vec<f64>, amplitudes: Vec<f64>) -> Self {
        let amplitudes = if amplitudes.len() < frequencies.len() {
            let needed = frequencies.len() - amplitudes.len();
            let mut a = amplitudes;
            a.extend(std::iter::repeat(1.0).take(needed));
            a
        } else {
            amplitudes
        };
        Self { frequencies, amplitudes }
    }

    /// Evaluate the chord's waveform at time t.
    pub fn evaluate(&self, t: f64) -> f64 {
        self.frequencies.iter().zip(self.amplitudes.iter())
            .map(|(&f, &a)| a * (2.0 * PI * f * t).sin())
            .sum()
    }
}

/// Measure consonance of a set of ternary values mapped to frequency ratios.
/// Consonance is 1 / (1 + sum of dissonance between all pairs).
/// Simple model: dissonance ∝ min(|ratio - simple_ratio|) for simple ratios like 1/1, 3/2, 4/3, etc.
pub fn consonance(frequencies: &[f64]) -> f64 {
    if frequencies.len() < 2 { return 1.0; }
    let simple_ratios: &[f64] = &[1.0, 6.0/5.0, 5.0/4.0, 4.0/3.0, 3.0/2.0, 5.0/3.0, 2.0];
    let mut total_diss = 0.0;
    let base = frequencies[0];
    if base <= 0.0 { return 0.0; }
    for &f in &frequencies[1..] {
        if f <= 0.0 { return 0.0; }
        let ratio = f / base;
        let min_dist = simple_ratios.iter()
            .map(|&sr| (ratio - sr).abs())
            .fold(f64::MAX, f64::min);
        total_diss += min_dist;
    }
    1.0 / (1.0 + total_diss)
}

/// Compute a dissonance curve over an interval range [1.0, max_ratio] at `steps` points.
/// Returns (ratio, consonance) pairs.
pub fn dissonance_curve(base_freq: f64, max_ratio: f64, steps: usize) -> Vec<(f64, f64)> {
    if steps == 0 { return vec![]; }
    (0..=steps).map(|i| {
        let ratio = 1.0 + (max_ratio - 1.0) * i as f64 / steps as f64;
        let freq2 = base_freq * ratio;
        let c = consonance(&[base_freq, freq2]);
        (ratio, c)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fundamental_basic() {
        let f = fundamental(100, 1000.0);
        assert!((f - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_fundamental_zero_len() {
        assert_eq!(fundamental(0, 1000.0), 0.0);
    }

    #[test]
    fn test_harmonic_series_count() {
        let h = harmonic_series(100.0, 5);
        assert_eq!(h.len(), 5);
        assert!((h[0] - 100.0).abs() < 1e-10);
        assert!((h[4] - 500.0).abs() < 1e-10);
    }

    #[test]
    fn test_subharmonic_basic() {
        let s = subharmonic(100.0, 2);
        assert!((s - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_subharmonic_zero_div() {
        assert_eq!(subharmonic(100.0, 0), 0.0);
    }

    #[test]
    fn test_overtone_index_second() {
        assert_eq!(overtone_index(200.0, 100.0), 2);
    }

    #[test]
    fn test_overtone_index_fundamental() {
        assert_eq!(overtone_index(100.0, 100.0), 1);
    }

    #[test]
    fn test_harmonic_distortion_pure() {
        let h = vec![1.0]; // only fundamental
        assert!((harmonic_distortion(&h)).abs() < 1e-10);
    }

    #[test]
    fn test_harmonic_distortion_with_overtones() {
        let h = vec![1.0, 0.5, 0.25];
        let d = harmonic_distortion(&h);
        assert!(d > 0.0);
        assert!(d < 1.0);
    }

    #[test]
    fn test_harmonic_distortion_empty() {
        assert_eq!(harmonic_distortion(&[]), 0.0);
    }

    #[test]
    fn test_chord_evaluate() {
        let c = Chord::new(vec![1.0], vec![1.0]);
        // sin(2π * 1 * 0) = 0
        assert!((c.evaluate(0.0)).abs() < 1e-10);
    }

    #[test]
    fn test_chord_amplitudes_pad() {
        let c = Chord::new(vec![100.0, 200.0], vec![0.5]);
        assert_eq!(c.amplitudes.len(), 2);
        assert!((c.amplitudes[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_consonance_single() {
        assert!((consonance(&[440.0]) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_consonance_octave() {
        let c = consonance(&[100.0, 200.0]);
        assert!(c > 0.9); // octave is very consonant
    }

    #[test]
    fn test_consonance_tritone() {
        let c_oct = consonance(&[100.0, 200.0]);
        let c_tri = consonance(&[100.0, 141.0]);
        assert!(c_oct > c_tri);
    }

    #[test]
    fn test_dissonance_curve_length() {
        let curve = dissonance_curve(100.0, 2.0, 50);
        assert_eq!(curve.len(), 51);
    }

    #[test]
    fn test_dissonance_curve_start() {
        let curve = dissonance_curve(100.0, 2.0, 10);
        assert!((curve[0].0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dissonance_curve_zero_steps() {
        let curve = dissonance_curve(100.0, 2.0, 0);
        assert!(curve.is_empty());
    }
}
