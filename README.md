# ternary-harmonic

**The overtone series, made from three values. Where physics meets Z₃.**

Play any note on any instrument and you don't hear one frequency — you hear a whole family of them. The fundamental, then 2x, 3x, 4x, 5x... the harmonic series. It's not a musical convention. It's physics. A vibrating string naturally produces all these frequencies simultaneously, and their relative amplitudes determine the *timbre* — the thing that makes a violin sound different from a flute playing the same note.

This crate maps the harmonic series into ternary space. Given a fundamental frequency, it generates overtones at integer multiples, each with configurable amplitude. The result is a rich, multi-layered ternary signal that carries the physics of real sound inside the {-1, 0, +1} constraint.

## What's Inside

- **`fundamental(freq, ticks)`** — the base tone. Same as `ternary_wave::square` but named for clarity
- **`overtone_series(freq, ticks, harmonics)`** — generate N harmonics. Each is a ternary wave at freq×n, with amplitude 1/n
- **`chord(frequencies, ticks)`** — play multiple frequencies simultaneously. Ternary addition (mod 3)
- **`interval(a, b, ticks)`** — two frequencies. The simplest chord. The relationship between them IS the music
- **`power_spectrum(signal)`** — compute the frequency content of a ternary signal. What frequencies are present?
- **`harmonic_ratio(signal)`** — how harmonic is the signal? Pure tones score high. Noise scores low
- **`dissonance(signals)`** — measure the sensory dissonance between multiple ternary signals

## Quick Example

```rust
use ternary_harmonic::*;

// A single fundamental at frequency 2
let fundamental = fundamental(2, 16);

// Add 4 overtones: 2f, 3f, 4f, 5f
let rich = overtone_series(2, 16, 4);
// A thicker, more complex sound than the fundamental alone

// A perfect fifth: frequencies 4 and 6 (ratio 3:2)
let fifth = interval(4, 6, 16);

// A major triad: 4, 5, 6 (ratio 4:5:6)
let major = chord(&[4, 5, 6], 16);

// Analyze: what's in this signal?
let spectrum = power_spectrum(&rich);
// Peaks at the fundamental and its multiples

// How consonant is the chord?
let diss = dissonance(&major);
// Low dissonance = sounds "nice". High = sounds "tense".
```

## The Deeper Truth

**The harmonic series is the same in ternary as it is in physics.** A string vibrates at its fundamental and all integer multiples. In continuous audio, each harmonic is a sine wave at fn with amplitude ∝ 1/n. In ternary, each harmonic is a quantized ternary wave at fn with amplitude 1/n (expressed as how often it reaches ±1 vs. 0).

The consequence: ternary harmonics have the *same ratios* as continuous harmonics. A perfect fifth (3:2) sounds like a fifth. An octave (2:1) sounds like an octave. The physics is preserved because the physics lives in the *ratios*, not the absolute values. Ternary quantization changes the timbre but preserves the harmonic relationships.

This is why ternary music works at all. The ear doesn't hear absolute amplitude — it hears frequency ratios. And ternary preserves those ratios. The "sound" is different (grittier, more digital) but the *harmony* is identical. You can play Bach in ternary. You can play Coltrane. The intervals are the same.

**Use cases:**
- **Additive synthesis** — build complex tones by stacking harmonics
- **Music theory** — teach harmonic relationships with the simplest possible representation
- **Algorithmic composition** — generate chords and intervals programmatically
- **Timbre design** — control which harmonics are present to shape the sound
- **Education** — the overtone series, made audible and manipulable

## See Also

- **ternary-wave** — the raw waveforms that harmonics are built from
- **ternary-resonance** — resonance as a dynamic process (harmonic filtering in real-time)
- **ternary-ear** — ear training: learn to hear these intervals
- **ternary-music** — music theory built on harmonic relationships
- **ternary-rack** — wire harmonic generators into a modular synth
- **ternary-fib** — the period-8 cycle as a natural harmonic rhythm

## Install

```bash
cargo add ternary-harmonic
```

## License

MIT
