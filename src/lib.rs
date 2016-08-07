pub fn insertsort(s: &str) -> String {
    let mut st = String::with_capacity(s.len());
    let mut si = s.chars().into_iter();
    match si.next() {
        Some(ch) => st.push(ch),
        None => return st,
    }
    while let Some(ch) = si.next() {
        let mut found = None;
        for (idx, och) in st.chars().enumerate() {
            if och >= ch {
                found = Some(idx);
                break
            }
        }
        if let Some(idx) = found {
            st.insert(idx, ch)
        } else {
            st.push(ch)
        }
    }
    st
}

pub fn countsort_insert(s: &str) -> String {
    let mut buckets = [0usize; 128];
    let mut nonascii = String::new();
    for ch in s.chars() {
        if ch < (128 as char) {
            buckets[ch as u32 as usize] += 1
        } else {
            let mut found = None;
            for (idx, och) in nonascii.chars().enumerate() {
                if och >= ch {
                    found = Some(idx);
                    break
                }
            }
            if let Some(idx) = found {
                nonascii.insert(idx, ch)
            } else {
                nonascii.push(ch)
            }
        }
    }
    let mut st = String::with_capacity(s.len());
    for (chcode, &bucket) in buckets.iter().enumerate() {
        let ch = chcode as u8 as char;
        for _ in 0..bucket {
            st.push(ch)
        }
    }
    st.push_str(&nonascii);
    st
}

pub fn countsort_vec(s: &str) -> String {
    let mut buckets = [0usize; 128];
    let mut nonascii = Vec::new();
    for ch in s.chars() {
        if ch < (128 as char) {
            buckets[ch as u32 as usize] += 1
        } else {
            nonascii.push(ch)
        }
    }
    let mut st = String::with_capacity(s.len());
    for (chcode, &bucket) in buckets.iter().enumerate() {
        let ch = chcode as u8 as char;
        for _ in 0..bucket {
            st.push(ch)
        }
    }
    nonascii.sort();
    for ch in nonascii {
        st.push(ch)
    }
    st
}

pub fn vecsort(s: &str) -> String {
    let mut charvec = s.chars().collect::<Vec<char>>();
    charvec.sort();
    charvec.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use std::cmp;
    fn teststr1<F>(s: &str, f: F) -> (String, Duration)
        where F: Fn(&str) -> String
    {
        let start = Instant::now();
        let st = f(s);
        (st, start.elapsed())
    }

    fn teststr(s: &str) {
        let (countins, countinsdur) = teststr1(s, countsort_insert);
        let (countvec, countvecdur) = teststr1(s, countsort_vec);
        let (insert, insertdur) = teststr1(s, insertsort);
        let (vec, vecdur) = teststr1(s, vecsort);
        assert!(countins == vec);
        assert!(countvec == vec);
        assert!(insert == vec);
        if s.len() != 64 {
            assert!(cmp::min(cmp::min(countinsdur, countvecdur), insertdur) <= vecdur);
        }
    }

    #[test]
    fn test8() {
        teststr("asdfhjkl");
    }

    #[test]
    fn test64() {
        teststr("asdfhjklasdfhjklasdfhjklasdfhjklasdfhjklasdfhjklasdfhjklasdfhjkl");
    }

    #[test]
    fn test4096() { 
        let mut s = String::with_capacity(4096);
        for i in 0..4096 {
            s.push((i * 733 % 26) as u8 as char);
        }
        teststr(&s);
    }
}
