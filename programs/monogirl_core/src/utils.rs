use sha2::{Sha256, Digest};

/// Optimized hash chain verification with pre-allocation
pub fn compute_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn combine_hashes(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

pub fn verify_hash_chain(hashes: &[[u8; 32]]) -> [u8; 32] {
    if hashes.is_empty() {
        return [0u8; 32];
    }

    let mut current = hashes[0];
    for hash in &hashes[1..] {
        current = combine_hashes(&current, hash);
    }
    current
}

pub fn compute_bundle_hash(
    merkle_root: &[u8; 32],
    epoch: u64,
    slot: u64,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(merkle_root);
    hasher.update(epoch.to_le_bytes());
    hasher.update(slot.to_le_bytes());
    hasher.finalize().into()
}

pub fn check_account_overlap(
    set_a: &[[u8; 32]],
    set_b: &[[u8; 32]],
) -> bool {
    for a in set_a {
        for b in set_b {
            if a == b {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() {
        let data = b"test data";
        let hash = compute_hash(data);
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn test_combine_hashes_deterministic() {
        let a = compute_hash(b"a");
        let b = compute_hash(b"b");
        let result1 = combine_hashes(&a, &b);
        let result2 = combine_hashes(&a, &b);
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_combine_hashes_order_matters() {
        let a = compute_hash(b"a");
        let b = compute_hash(b"b");
        let ab = combine_hashes(&a, &b);
        let ba = combine_hashes(&b, &a);
        assert_ne!(ab, ba);
    }

    #[test]
    fn test_verify_hash_chain_empty() {
        let result = verify_hash_chain(&[]);
        assert_eq!(result, [0u8; 32]);
    }

    #[test]
    fn test_bundle_hash() {
        let root = [1u8; 32];
        let hash = compute_bundle_hash(&root, 100, 500);
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn test_account_overlap_detected() {
        let shared = [42u8; 32];
        let set_a = vec![[1u8; 32], shared];
        let set_b = vec![shared, [2u8; 32]];
        assert!(check_account_overlap(&set_a, &set_b));
    }

    #[test]
    fn test_no_account_overlap() {
        let set_a = vec![[1u8; 32], [2u8; 32]];
        let set_b = vec![[3u8; 32], [4u8; 32]];
        assert!(!check_account_overlap(&set_a, &set_b));
    }
}
