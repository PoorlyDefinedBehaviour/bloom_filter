use std::{
  collections::HashSet,
  hash::{Hash, Hasher},
};

use rand::{distributions::Alphanumeric, Rng};

struct FnvHasher {
  hash: u64,
}

impl FnvHasher {
  pub fn new() -> Self {
    Self { hash: 2166136261 }
  }
}

impl Hasher for FnvHasher {
  fn write(&mut self, bytes: &[u8]) {
    for byte in bytes {
      self.hash ^= *byte as u64;
      self.hash = self.hash.wrapping_mul(16777619);
    }
  }

  fn finish(&self) -> u64 {
    self.hash as u64
  }
}

struct Hasher2 {
  hash: u64,
}

impl Hasher2 {
  pub fn new() -> Self {
    Self { hash: 0 }
  }
}

impl Hasher for Hasher2 {
  fn write(&mut self, bytes: &[u8]) {
    for byte in bytes {
      self.hash = self.hash.wrapping_add(*byte as u64);
    }
  }

  fn finish(&self) -> u64 {
    self.hash
  }
}

struct Hasher3 {
  hash: u64,
}

impl Hasher3 {
  pub fn new() -> Self {
    Self { hash: 1 }
  }
}

impl Hasher for Hasher3 {
  fn write(&mut self, bytes: &[u8]) {
    for (i, byte) in bytes.iter().enumerate() {
      self.hash = self
        .hash
        .wrapping_add((19_u64).wrapping_pow(i as u32).wrapping_mul(*byte as u64));
    }
  }

  fn finish(&self) -> u64 {
    self.hash
  }
}

#[derive(Debug)]
struct BloomFilter {
  elements: Vec<u8>,
}

impl BloomFilter {
  pub fn new(size: usize) -> Self {
    Self {
      elements: vec![0_u8; size],
    }
  }

  fn hash(&self, value: impl Hash) -> (usize, usize, usize) {
    let a = {
      let mut hasher = FnvHasher::new();
      value.hash(&mut hasher);
      hasher.finish()
    };

    let b = {
      let mut hasher = Hasher2::new();
      value.hash(&mut hasher);
      hasher.finish()
    };

    let c = {
      let mut hasher = Hasher3::new();
      value.hash(&mut hasher);
      hasher.finish()
    };

    let len = self.elements.len();

    (a as usize % len, b as usize % len, c as usize % len)
  }

  pub fn add(&mut self, value: impl Hash) {
    let (a, b, c) = self.hash(value);

    self.elements[a] += 1;
    self.elements[b] += 1;
    self.elements[c] += 1;
  }

  pub fn contains(&self, value: impl Hash) -> bool {
    let (a, b, c) = self.hash(value);
    self.elements[a] > 0 && self.elements[b] > 0 && self.elements[c] > 0
  }
}

fn random_string() -> String {
  rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect()
}

fn main() {
  let mut false_positive_found = false;

  // Try to find a false positive
  for i in 0..=100 {
    if false_positive_found {
      break;
    }

    let mut filter = BloomFilter::new(10000);

    let values: HashSet<String> = (0..=100).map(|_| random_string()).collect();

    for value in values {
      if filter.contains(&value) {
        println!("false positive found at iteration {}: {}", i, &value);
        false_positive_found = true;
      } else {
        filter.add(value);
      }
    }
  }
}
