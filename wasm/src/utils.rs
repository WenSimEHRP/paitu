use crate::types::ScaleMode;

pub fn match_and_scale(mode: &ScaleMode, length: f32, scale: f32) -> f32 {
    match mode {
        ScaleMode::Auto => length,
        ScaleMode::Linear => length,
        ScaleMode::Uniform => 1.0,
        ScaleMode::Square => length * length,
        ScaleMode::Logarithmic => length.ln(),
        ScaleMode::SquareRoot => length.sqrt(),
    }
    .max(1.0)
    * scale
}

/// Simple pseudo-random number generator using PCG algorithm
/// Takes a u32 seed and returns a u32 random number
pub fn simple_prng(seed: u32) -> u32 {
    // PCG constants
    const MULTIPLIER: u64 = 6364136223846793005;
    const INCREMENT: u64 = 1442695040888963407;

    // Convert seed to u64 for computation
    let state = seed as u64;

    // Linear congruential step
    let next_state = state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);

    // Output permutation (PCG-style)
    let xorshifted = (((next_state >> 18) ^ next_state) >> 27) as u32;
    let rotation = (next_state >> 59) as u32;

    // Rotate right
    xorshifted.rotate_right(rotation)
}

/// Generate a random number in a specific range [min, max)
pub fn prng_range(seed: u32, min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    let range = max - min;
    min + (simple_prng(seed) % range)
}
