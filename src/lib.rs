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

pub fn bucketsort(s: &str) -> String {
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

pub fn vecsort(s: &str) -> String {
    let mut charvec = s.chars().collect::<Vec<char>>();
    charvec.sort();
    charvec.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use std::cmp;
    fn teststr(s: &str) {
        let strstart = Instant::now();
        let strstr = bucketsort(s);
        let strtime = strstart.elapsed();
        let insertstart = Instant::now();
        let insertstr = insertsort(s);
        let inserttime = insertstart.elapsed();
        let vecstart = Instant::now();
        let vecstr = vecsort(s);
        let vectime = vecstart.elapsed();
        assert!(strstr == vecstr);
        assert!(insertstr == vecstr);
        if s.len() != 64 {
            assert!(cmp::min(strtime, inserttime) <= vectime);
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
