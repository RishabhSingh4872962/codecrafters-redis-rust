
pub fn parser(str: &str) -> Vec<&str> {
    let first_ch: &str = &str[..1];

    let mut v: Vec<&str> = Vec::new();

    // println!("str parser===>{}",str);
    match first_ch {
        "*" => {
            let next = str.find("\r\n");

            if let Some(index) = next {
                // let mut arr_len: isize = str[1..index].parse().unwrap();

                let rest_str = &str[index + 2..];

                // println!("arr_len==> {},res_str ===>{:?}", 0, rest_str);

                let mut p = 0;

                for s in rest_str.lines() {
                    handle_string(s, &mut p, &mut v);
                }
            }

            return v;
        }
        _ => return v,
    }
}

pub fn handle_string<'a>(str: &'a str, prev: &mut usize, v: &mut Vec<&'a str>) {
    let first_ch = &str[..1];

    match first_ch {
        "$" => {
            *prev = str[1..].parse().unwrap();
        }
        _ if str.len() > 0 => {
            if str.len() == *prev {
                v.push(str);
            }
        }
        _ => {}
    }
}
