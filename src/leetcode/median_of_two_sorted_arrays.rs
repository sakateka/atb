use std::mem;

/// Алгоритм поиска
/// Есть 2 отсортирванных массива, допустим
/// a:   [7 8 9 13 55 80 81 84 300]
/// b:         [12 14 15 20 28 33 55 60 83 85 89]
///
/// Нужно найти медиану этих массивов.
///
/// Простой подход, требдующий O(n log n) и 2х по памяти
///                                         ,.- cередина массива
///                                         v   медиана равна (33+55) == 44
/// c = a + b == [7 8 9 12 13 14 15 20 28 33 55 55 60 80 81 83 84 85 89 300]
/// c.sort()
/// m = (c[(len - 1)/2] + c[len/2]) / 2 == 44
pub fn find_median_sorted_arrays_naive(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
    let mut nums3 = nums1.clone();
    nums3.extend(nums2);
    nums3.sort();
    median(&nums3)
}

fn median(a: &[i32]) -> f64 {
    let len = a.len();
    if len == 0 {
        return 0.0;
    }
    let m = a[len / 2] as f64;
    if len % 2 == 1 {
        m
    } else {
        (m + a[(len - 1) / 2] as f64) / 2.0
    }
}

pub fn find_median_sorted_arrays_naive_without_extra_mem(
    nums1: &Vec<i32>,
    nums2: &Vec<i32>,
) -> f64 {
    let len = nums1.len() + nums2.len();
    if len == 0 {
        return 0.0;
    }
    if nums1.len() == 0 {
        return median(&nums2);
    }
    if nums2.len() == 0 {
        return median(&nums1);
    }

    let middle = len / 2;
    let mut counter: usize = 0;

    let mut e1: i32 = i32::min(nums1[0], nums2[0]);
    let mut e2: i32 = e1;

    let mut i: usize = 0;
    let mut j: usize = 0;
    while counter <= middle {
        let step_i;
        let step_j;

        step_j = j < nums2.len();
        step_i = i < nums1.len() && (!step_j || nums1[i] <= nums2[j]);

        if !step_i && !step_j {
            break;
        }
        if step_i {
            e2 = e1;
            e1 = nums1[i];
            i += 1;
        } else if step_j {
            e2 = e1;
            e1 = nums2[j];
            j += 1;
        }
        counter += 1;
    }

    if len % 2 == 1 {
        e1 as f64
    } else {
        (e1 + e2) as f64 / 2.0
    }
}

/// Подход требующий O(log n) без доп памяти
/// Алгортм заключается в постоянном разделении массивов на правую и левую части
pub fn find_median_sorted_arrays_fast(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
    let len = nums1.len() + nums2.len();
    let half = (len as f64 / 2.0).floor() as isize;

    let (mut a, mut b) = (nums1, nums2);
    if a.len() > b.len() {
        mem::swap(&mut a, &mut b);
    }
    if a.len() == 0 {
        return median(b);
    }

    let (mut l, mut r) = (0 as isize, a.len() as isize - 1);
    loop {
        let i: isize = ((l + r) as f64 / 2.0).floor() as isize;
        let j: isize = half - i - 2;

        let a_left = if i >= 0 {
            a[i as usize] as f64
        } else {
            f64::NEG_INFINITY
        };
        let a_right = if ((i + 1) as usize) < a.len() {
            a[(i + 1) as usize] as f64
        } else {
            f64::INFINITY
        };

        let b_left = if j >= 0 {
            b[j as usize] as f64
        } else {
            f64::NEG_INFINITY
        };

        let b_right = if ((j + 1) as usize) < b.len() {
            b[(j + 1) as usize] as f64
        } else {
            f64::INFINITY
        };

        if a_left <= b_right && b_left <= a_right {
            return if len % 2 == 1 {
                a_right.min(b_right)
            } else {
                (a_left.max(b_left) + a_right.min(b_right)) / 2.0
            };
        } else if a_left > b_right {
            r = i - 1;
        } else {
            l = i + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use rand::Rng;

    fn tests_for() -> HashMap<&'static str, fn(&Vec<i32>, &Vec<i32>) -> f64> {
        HashMap::from([
            (
                "naive",
                find_median_sorted_arrays_naive as fn(&Vec<i32>, &Vec<i32>) -> f64,
            ),
            (
                "naive_without_extra_mem",
                find_median_sorted_arrays_naive_without_extra_mem,
            ),
            ("fast", find_median_sorted_arrays_fast),
        ])
    }

    #[test]
    fn test_find_median_in_sorted_arrays_all() {
        struct TestCase {
            name: &'static str,
            a: Vec<i32>,
            b: Vec<i32>,
            expected: f64,
        }
        let cases = vec![
            TestCase {
                name: "simple",
                a: vec![7, 8, 9, 13, 55, 80, 81, 84, 300],
                b: vec![12, 14, 15, 20, 28, 33, 55, 60, 83, 85, 89],
                expected: 44.0,
            },
            TestCase {
                name: "b[2]",
                a: vec![1, 3],
                b: vec![2],
                expected: 2.0,
            },
            TestCase {
                name: "a,b",
                a: vec![1, 2],
                b: vec![3, 4],
                expected: 2.5,
            },
            TestCase {
                name: "all zero",
                a: vec![0, 0],
                b: vec![0, 0],
                expected: 0.0,
            },
            TestCase {
                name: "a empty",
                a: vec![],
                b: vec![1],
                expected: 1.0,
            },
            TestCase {
                name: "b empty",
                a: vec![2],
                b: vec![],
                expected: 2.0,
            },
            TestCase {
                name: "a single element",
                a: vec![34],
                b: vec![
                    -101, -95, -67, -38, -38, -36, -34, -19, -6, 13, 14, 28, 45, 52, 54, 57, 64,
                    73, 90, 95, 95, 107,
                ],
                expected: 28.0,
            },
            TestCase {
                name: "a single element",
                a: vec![-93, -43, -15, 4, 46, 91, 106, 109],
                b: vec![-76, -66, 53, 70],
                expected: 25.0,
            },
        ];
        for (n, f) in tests_for() {
            for c in cases.iter() {
                eprintln!("Run test {}: {}", n, c.name);
                assert_eq!(c.expected, f(&c.a, &c.b))
            }
        }
    }

    #[test]
    fn test_find_median_in_sorted_arrays_naive_vs_fast() {
        let mut rng = rand::thread_rng();
        let cases = (0..50)
            .map(|_| {
                (
                    {
                        let mut a: Vec<i32> = (0..rng.gen::<u8>() / 10)
                            .map(|_| rng.gen::<i8>() as i32)
                            .collect();
                        a.sort();
                        a
                    },
                    {
                        let mut b: Vec<i32> = (0..rng.gen::<u8>() / 10)
                            .map(|_| rng.gen::<i8>() as i32)
                            .collect();
                        b.sort();
                        b
                    },
                )
            })
            .collect::<Vec<(Vec<i32>, Vec<i32>)>>();
        for (idx, (a, b)) in cases.iter().enumerate() {
            assert_eq!(
                find_median_sorted_arrays_naive(a, b),
                find_median_sorted_arrays_fast(a, b),
                "\ncase #{}\na: {:?}\nb: {:?}\n",
                idx,
                a,
                b,
            )
        }
    }
}
