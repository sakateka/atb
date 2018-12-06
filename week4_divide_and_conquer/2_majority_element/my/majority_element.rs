use std::io::{self, Read};

fn get_majority_nlogn(a: &[u32]) -> Option<u32> {
    let a_len = a.len();
    if a_len == 0 {
        return None;
    }

    if a_len == 1 {
        return Some(a[0]);
    }

    if a_len == 2 {
        if a[0] == a[1] {
            return Some(a[0]);
        } else {
            return None;
        }
    }

    let mid = a_len/2;
    let left = get_majority_nlogn(&a[..mid]);
    let right = get_majority_nlogn(&a[mid..]);

    if left == right {
        return left;
    }

    let mut l_count = 0;
    if let Some(left) = left {
        l_count = a.iter().filter(|x| left == **x).count();
    }
    let mut r_count = 0;
    if let Some(right) = right {
        r_count = a.iter().filter(|x| right == **x).count();
    }
    let (major, el) = if l_count > r_count  { (l_count, left) } else { (r_count, right) };
    if major > mid {
        return el;
    }
    None
}


fn get_majority_fast(a: &[u32]) -> Option<u32> {
    if a.len() == 0 {
        return None
    }

    let mut count:i64 = 0;
    let mut candidate = a[0];
    for i in a {
        if count == 0 {
            candidate = *i;
        }
        count += if candidate == *i { 1 } else { -1 }
    }
    count = 0;
    for i in a {
       if candidate == *i {
           count += 1
       }
    }
    if count > 0 && count as usize > a.len()/2 {
        Some(candidate)
    } else {
        None
    }
}

fn get_majority_naive(a: &[u32]) -> Option<u32> {
    for i in a {
        let mut count = 0;
        for j in a {
            if j == i {
                count += 1;
            }
        }
        if count > a.len() / 2 {
            return Some(*i as u32)
        }
    }
    None
}


pub fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let numbers: Vec<u32> = buf.trim().split_whitespace().skip(1)
        .map(|x| x.parse().unwrap()).collect();

    if let Some(_) = get_majority_fast(&numbers[..]) {
        println!("{}", 1);
    } else {
        println!("{}", 0);
    }
}


#[test]
fn get_majority_naive_test() {
    struct TestCase {
        result: Option<u32>,
        elements: Vec<u32>,
    }
    let manual_tests = vec![
        TestCase{result: Some(1), elements: vec![1]},
        TestCase{result: Some(2), elements: vec![2, 2]},
        TestCase{result: Some(3), elements: vec![3, 3, 3]},
        TestCase{result: None,    elements: vec![1, 2, 3]},
        TestCase{result: None,    elements: vec![1, 2, 3, 4]},
        TestCase{result: None,    elements: vec![1, 2, 3, 1]},
        TestCase{result: Some(2), elements: vec![1, 2, 2, 2]},
        TestCase{result: Some(2), elements: vec![2, 1, 2, 2]},
        TestCase{result: Some(2), elements: vec![2, 2, 1, 2]},
        TestCase{result: Some(2), elements: vec![2, 2, 2, 1]},
        TestCase{result: Some(2), elements: vec![2, 2, 2, 1, 1]},
        TestCase{result: None,    elements: vec![2, 2, 2, 1, 1, 1]},
        TestCase{result: Some(2), elements: vec![1, 1, 2, 2, 2]},
        TestCase{result: Some(2), elements: vec![2, 3, 9, 2, 2]},
        TestCase{result: None,    elements: vec![1, 2, 3, 1]},
    ];
    for c in manual_tests {
        assert_eq!(c.result, get_majority_naive(&c.elements[..]), "{:?}", c.elements);
    }
}

#[test]
fn get_majority_fast_test() {
    use std::time::Instant;

    fn random() -> u32 {
      unsafe {
        rand()
      }
    }

    extern "C" {
      fn rand() -> u32;
    }


    let mut cases = vec![
        vec![2, 3, 9, 2, 2],
    ];

    for i in 0..100 {
        let major = random();
        let len = if i % 2 == 0 {
            random() % (4*(u32::pow(10, 4)))
        } else {
            random() % 10
        } as usize;

        let cases_len = cases.len();
        let seq: Vec<u32> = if cases_len > 0 {
           cases.remove(0) 
        } else {
            (0..len*2)
            .map(|x| if x % 2 == 0 {major} else {random()})
            .collect()
        };

        let now = Instant::now();
        let nm = get_majority_naive(&seq[..]);
        let end = now.elapsed();
        let naive_time  = end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9;
        let now = Instant::now();
        let fm = get_majority_fast(&seq[..]);
        let end = now.elapsed();
        let fast_time  = end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9;
        let now = Instant::now();
        let dm = get_majority_nlogn(&seq[..]);
        let end = now.elapsed();
        let nlogn_time  = end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9;
        println!("len = {}, naive_time = {:0.3}, fast_time = {:0.6}, nlogn_time = {:0.6}",
                 seq.len(), naive_time, fast_time, nlogn_time);
        assert!(nm == fm && fm == dm, "{:?}", (nm, fm, dm));
    }
}
