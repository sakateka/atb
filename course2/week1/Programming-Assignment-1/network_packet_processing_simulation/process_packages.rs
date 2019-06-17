use std::io;
use std::collections::VecDeque;

pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let size = buf.split_whitespace().nth(0).unwrap().parse::<usize>().unwrap();
    buf.clear();
    let mut stack = VecDeque::with_capacity(size);

    while io::stdin().read_line(&mut buf).unwrap() > 0 {
        {
            let a = buf.split_whitespace().take(2).collect::<Vec<&str>>();
            let (arrival, duration) = (a[0].parse::<i32>().unwrap(),
                                       a[1].parse::<i32>().unwrap());
            let mut last = stack.pop_front();
            while let Some(complete_time) = last {
                if complete_time <= arrival {
                    last = stack.pop_front();
                } else {
                    stack.push_front(complete_time);
                    break;
                }
            }

            let mut last_start_time = arrival;
            if let Some(last_by_stack) = stack.back() {
                last_start_time = *last_by_stack;
            }

            if stack.len() >= size  {
                last_start_time = -1;
            } else {
                stack.push_back(last_start_time + duration);
            }
            println!("{}", last_start_time);
        }
        buf.clear();
    }
}
