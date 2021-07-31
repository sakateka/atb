/// ```
/// use algorithmic_toolbox::sort::insertion::sort;
///
/// let sorted = vec![1, 2, 3, 4, 5];
/// let mut v = vec![5, 4, 3, 2, 1];
/// sort(&mut v);
/// v = vec![2, 1, 3, 5, 4];
/// sort(&mut v);
/// v = vec![1, 2, 3, 4, 5];
/// sort(&mut v);
/// assert_eq!(v, sorted);
/// ```
pub fn sort(arr: &mut [u32]) {
    let len = arr.len();
    for j in 1..len {
        let key = arr[j];
        let mut i = j;
        while i > 0 && arr[i - 1] > key {
            arr[i] = arr[i - 1];
            i -= 1;
        }
        arr[i] = key
    }
}
