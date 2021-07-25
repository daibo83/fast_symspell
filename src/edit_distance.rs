use triple_accel::levenshtein::rdamerau_exp;
// use strsim::{damerau_levenshtein, hamming, jaro, jaro_winkler, levenshtein, osa_distance};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DistanceAlgorithm {
    Damerau,
}

pub struct EditDistance {
    algorithm: DistanceAlgorithm,
}

impl EditDistance {
    pub fn new(distance_algorithm: DistanceAlgorithm) -> EditDistance {
        EditDistance {
            algorithm: distance_algorithm,
        }
    }

    pub fn compare(&self, string: &str, other: &str, max_distance: i64) -> i64 {
        let distance = match self.algorithm {
            DistanceAlgorithm::Damerau => rdamerau_exp(string.as_bytes(), other.as_bytes()),
        };

        if distance <= max_distance as u32 {
            distance as i64
        } else {
            -1
        }
    }
}
