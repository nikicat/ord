use super::*;
use crate::runes::MAX_DIVISIBILITY;

pub(crate) enum Chunkiness {
  Fine,
  Sand,
  Peebles,
  Rocks,
  Chongus,
}

impl From<f64> for Chunkiness {
  fn from(chunk_factor: f64) -> Self {
    match chunk_factor {
      f if f >= 1.0 && f < 2.0 => Self::Fine,
      f if f >= 2.0 && f < 3.0 => Self::Sand,
      f if f >= 3.0 && f < 4.0 => Self::Peebles,
      f if f >= 4.0 && f < 5.0 => Self::Rocks,
      _ => Self::Chongus,
    }
  }
}

impl Chunkiness {
  pub(crate) fn calculate_factor(supply: u64, divisibility: u8) -> f64 {
    // (((divisibility + 1) * supply) as f64).log10()

    assert!(divisibility <= MAX_DIVISIBILITY);


    let total_unit_count = supply.saturating_mul(10_u64.pow(divisibility as u32));

    total_unit_count as f64
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn chunkiness() {
    println!("Chunkiest: {}", Chunkiness::calculate_factor(1, 0));
    println!("Bitcoin: {}", Chunkiness::calculate_factor(21_000_000, 8));
    println!(
      "Fine: {}",
      Chunkiness::calculate_factor(1, MAX_DIVISIBILITY)
    );
  }
}
