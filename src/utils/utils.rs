use std::{collections::vec_deque::Iter, time::{Duration, Instant}};

pub fn handle_negative_index(mut start: isize, mut end: isize, len: isize) -> (usize, usize) {
    if start < 0 {
        start = start + len;

        if start < 0 {
            start = 0;
        }
    }
    if end < 0 {
        end = end + len;
        if end < 0 {
            end = 0;
        }
    }

    if end >= len {
        end = len - 1;
    }

    (start as usize, end as usize)
}

//  a,  b,  c,  d,   e,
//  0   1   2   3    4
//  -5  -4  -3  -2  -1

//   0  2  => a,b,c
//   0  8  => a,b,c,d,e   8 change to arr length -1  =>4

//   0 -1    0..=4;
//
//  -2  -5   3  0

pub fn create_array_response(v: Iter<String>) -> String {
    let mut res = format!("*{}\r\n", v.len());

    for ele in v.into_iter() {
        let s = create_string_response(ele);

        res.push_str(&s);
    }

    res
}

pub fn create_string_response(str: &str) -> String {
    format!("${}\r\n{}\r\n", str.len(), str)
}

pub fn handle_expiry(expiry_type: &str, time: u64) -> Instant {
    match expiry_type {
        "PX" => Instant::now() + Duration::from_millis(time),

        "EX" => Instant::now() + Duration::from_secs(time),
        _ => Instant::now(),
    }
}
