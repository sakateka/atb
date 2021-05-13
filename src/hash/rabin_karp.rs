const PRIME: usize = 1_000_000_007;
const X: usize = 263;

fn poly_hash(text: &[u8]) -> usize {
    let mut hash = 0;
    for c in text.iter().rev() {
        hash = (hash*X + *c as usize) % PRIME
    }
    hash
}

fn precompute_hashes(text: &[u8], pattern_len: usize) -> Vec<usize> {
    let text_len = text.len();
    let mut result = vec![0;text_len - pattern_len + 1];
    let s = &text[(text_len - pattern_len)..text_len];
    result[text_len - pattern_len] = poly_hash(s);
    let mut y = 1_usize;
    for _ in 0..pattern_len {
        y = (y * X) % PRIME;
    }
    for i in (0..(text_len - pattern_len)).rev() {
        let sub = y * (text[i+pattern_len] as usize) % PRIME;
        let add = text[i] as isize - sub as isize;
        // avoid (-2)%5 != 3%5 by x = ((a%p) + p)%p instead of just x = a%p.
        let add = (add + PRIME as isize) as usize % PRIME;
        result[i] = (X * result[i+1] + add) % PRIME;
    }
    return result;
}

fn are_equal(s1: &[u8], s2: &[u8]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            return false;
        }
    }
    true
}

/// ```
/// use algorithms::hash::rabin_karp::rabin_karp;
///
/// let text = "abcbabc";
/// let pattern = "ab";
/// let result = rabin_karp(text.as_bytes(), pattern.as_bytes());
/// assert_eq!(result, vec![0, 4]);
///
/// let text = "textTexttexT";
/// let pattern = "Text";
/// let result = rabin_karp(text.as_bytes(), pattern.as_bytes());
/// assert_eq!(result, vec![4]);
///
/// let text = "textTexttexT";
/// let pattern = "NotFound";
/// let result = rabin_karp(text.as_bytes(), pattern.as_bytes());
/// assert_eq!(result, vec![]);
/// ```
pub fn rabin_karp(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut result = Vec::new();
    let p_hash = poly_hash(pattern);
    let t_hashes = precompute_hashes(text, pattern.len());
    for i in 0..(text.len() - pattern.len() + 1) {
        if p_hash != t_hashes[i] {
            continue;
        }
        if are_equal(&text[i..(i+pattern.len())], pattern) {
            result.push(i);
        }
    }
    return result;
}
