pub fn length_of_longest_substring(s: String) -> i32 {
    let mut len: i32 = 0;
    let mut start_idx: i32 = 0;
    let mut last_idx: i32;
    let mut index: [i32; 256] = [-1; 256];

    for (i, c) in s.into_bytes().iter().enumerate() {
        last_idx = index[(*c) as usize];
        if last_idx > -1 {
            start_idx = i32::max(start_idx, last_idx + 1);
        }
        len = i32::max(len, i as i32 - start_idx + 1);
        index[*c as usize] = i as i32;
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("bb".to_string()), 1);
        assert_eq!(length_of_longest_substring("aa".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
        assert_eq!(length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    }
}
