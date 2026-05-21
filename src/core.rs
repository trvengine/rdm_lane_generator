// ============================================================
//  RDM™ LANE-BALANCED PRIME GENERATOR
//  Uchechukwu RDI Law (First Law) — Spectral Gap Δ=1
//  Developed by TRV™ Labs — Zero External Dependencies
// ============================================================

/// Lane tags based on mod 6
#[derive(Debug, Clone, PartialEq)]
pub enum Lane {
    LMinus,  // 6k - 1 (L-)
    LPlus,   // 6k + 1 (L+)
    Special, // 2 or 3
}

/// A tested prime with its lane coordinates
pub struct TaggedPrime {
    pub value: u64,
    pub lane: Lane,
    pub k: u64,
}

/// LEI-based prime test (4-form Lattice Exclusion Identity)
pub fn is_prime_lei(n: u64) -> bool {
    if n <= 1 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let r6 = n % 6;
    if r6 != 1 && r6 != 5 { return false; }

    if r6 == 1 {
        let k = (n - 1) / 6;
        let mut a: u64 = 1;
        loop {
            let dm = 6 * a - 1;
            if let Some(sq) = dm.checked_mul(dm) {
                if sq > n { break; }
            } else { break; }
            let dp = 6 * a + 1;
            if k >= 6 * a * a + 2 * a {
                if (k - a) % dp == 0 { return false; }
            }
            if k >= 6 * a * a - 2 * a {
                if (k + a) % dm == 0 { return false; }
            }
            a += 1;
        }
    } else {
        let k = (n + 1) / 6;
        let mut a: u64 = 1;
        loop {
            let dm = 6 * a - 1;
            if let Some(sq) = dm.checked_mul(dm) {
                if sq > n { break; }
            } else { break; }
            let dp = 6 * a + 1;
            if k >= 5 * a + 1 {
                if (k + a) % dp == 0 { return false; }
            }
            if k >= 7 * a - 1 {
                if (k - a) % dm == 0 { return false; }
            }
            a += 1;
        }
    }
    true
}

/// Generator state for producing lane-balanced primes
pub struct RDMLaneGenerator {
    pub current_k: u64,
    pub test_plus_next: bool,
    pub l_minus_count: u64,
    pub l_plus_count: u64,
    pub total_count: u64,
}

impl RDMLaneGenerator {
    pub fn new(start_n: u64) -> Self {
        let current_k = if start_n <= 5 { 1 } else { start_n / 6 };
        RDMLaneGenerator {
            current_k,
            test_plus_next: false,
            l_minus_count: 0,
            l_plus_count: 0,
            total_count: 0,
        }
    }

    /// Generate the next prime
    pub fn next_prime(&mut self) -> TaggedPrime {
        loop {
            if !self.test_plus_next {
                let candidate = 6 * self.current_k - 1;
                self.test_plus_next = true;
                if is_prime_lei(candidate) {
                    self.l_minus_count += 1;
                    self.total_count += 1;
                    return TaggedPrime { value: candidate, lane: Lane::LMinus, k: self.current_k };
                }
            } else {
                let candidate = 6 * self.current_k + 1;
                let k = self.current_k;
                self.test_plus_next = false;
                self.current_k += 1;
                if is_prime_lei(candidate) {
                    self.l_plus_count += 1;
                    self.total_count += 1;
                    return TaggedPrime { value: candidate, lane: Lane::LPlus, k };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lane_tags() {
        let mut generator = RDMLaneGenerator::new(5);
        let p1 = generator.next_prime();
        assert_eq!(p1.value, 5);
        assert_eq!(p1.lane, Lane::LMinus);
        
        let p2 = generator.next_prime();
        assert_eq!(p2.value, 7);
        assert_eq!(p2.lane, Lane::LPlus);
        
        let p3 = generator.next_prime();
        assert_eq!(p3.value, 11);
        assert_eq!(p3.lane, Lane::LMinus);
        
        let p4 = generator.next_prime();
        assert_eq!(p4.value, 13);
        assert_eq!(p4.lane, Lane::LPlus);
    }

    #[test]
    fn test_balance() {
        let mut generator = RDMLaneGenerator::new(5);
        for _ in 0..100 {
            generator.next_prime();
        }
        // Both lanes should have some primes (50/50 asymptotically)
        assert!(generator.l_minus_count > 30);
        assert!(generator.l_plus_count > 30);
    }
}
