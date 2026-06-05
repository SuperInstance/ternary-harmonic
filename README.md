# ternary-harmonic

**Harmonic series, overtones, and chords for ternary signals.**

Every periodic signal has a fundamental frequency — the base repetition rate — and overtones at integer multiples. The harmonic series is the physics of *why* a violin and a trumpet playing the same note sound different: they share the fundamental but have different overtone amplitudes.

This crate brings harmonic analysis to ternary signals. Given a repeating pattern of `{-1, 0, +1}`, compute the fundamental, generate the harmonic series, measure harmonic distortion, and build chords from ternary frequencies.

## What's Inside

- **`fundamental(pattern_len, sample_rate)`** — frequency of a repeating ternary pattern
- **`harmonic_series(base_freq, count)`** — overtones at n×f for n = 1..count
- **`subharmonic(base_freq, n)`** — frequency at 1/n of the fundamental
- **`overtone_index(frequency, base_freq)`** — which harmonic number is closest?
- **`harmonic_distortion(harmonics)`** — THD: ratio of non-fundamental energy to total
- **`Chord`** — multiple frequencies with amplitudes, weighted sum, and consonance measure
- **`consonance(freqs)`** — how harmonically compatible are these frequencies? (Based on simple integer ratios)

## Quick Example

```rust
use ternary_harmonic::*;

// A ternary pattern repeats every 8 samples at 8000 Hz
let fund = fundamental(8, 8000.0);
assert_eq!(fund, 1000.0); // 1 kHz fundamental

// First 5 harmonics
let harmonics = harmonic_series(fund, 5);
// [1000, 2000, 3000, 4000, 5000] Hz

// Which harmonic is 3000 Hz?
assert_eq!(overtone_index(3000.0, fund), 3); // 3rd harmonic

// Build a chord: root + fifth + octave
let chord = Chord::new(
    vec![100.0, 150.0, 200.0],  // frequencies
    vec![1.0, 0.7, 0.5],        // amplitudes
);
let cons = consonance(&chord.frequencies);
// High consonance: 1:1.5:2 = simple integer ratios

// Measure distortion
let h = vec![1.0, 0.3, 0.1, 0.05]; // fundamental + overtones
let thd = harmonic_distortion(&h);
// ~10% distortion — the overtones add flavor, not noise
```

## The Insight

**Ternary patterns have natural harmonics.** A repeating ternary sequence of period 8 (the Fibonacci period!) has a fundamental frequency and a specific overtone structure. The ternary constraint means harmonics are *discrete* — there's no spectral leakage, no windowing artifacts. The harmonic content is exactly determined by the pattern shape.

**Use cases:**
- **Audio synthesis** — build rich timbres from simple ternary patterns
- **Music theory** — harmonic analysis of algorithmic compositions
- **Signal analysis** — identify harmonic structure in ternary data streams
- **Acoustic modeling** — simulate room resonances with ternary-valued modes
- **Education** — demonstrate harmonic series with minimal state

## See Also

- **ternary-fib** — period-8 ternary Fibonacci (a natural harmonic foundation)
- **ternary-wave** — waveform generators that create harmonic content
- **ternary-phase** — phase relationships between harmonics
- **ternary-envelope** — shape the amplitude of harmonic content over time
- **ternary-echo** — echoes create comb-filter harmonics

## Install

```bash
cargo add ternary-harmonic
```

## License

MIT
