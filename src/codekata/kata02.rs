//! Implements <http://codekata.com/kata/kata02-karate-chop/>

pub fn main() {
    println!("Hello from Kata 02");
}

/// Simple recursive implementation of binary chop.
///
/// Compare middle element of `haystack` to `needle` parameter and then
/// return immediately or descend left or right. Note that the returned
/// index is always 0-based, so if we descend right we need to add the
/// middle index as an offset to the returned value.
pub fn chop_recursive(needle: i32, haystack: &[i32]) -> Option<usize> {
    match haystack {
        [] => None,

        // strictly speaking we don't need this branch
        // because the mid_elem check below covers it
        // as well
        [x] if *x == needle => Some(0),

        [_] => None,

        _ => {
            let mid_index = (haystack.len() - 1) / 2;
            let mid_elem = haystack[mid_index];
            if mid_elem == needle {
                Some(mid_index)
            } else if mid_elem < needle {
                // the needle can only be in the right part, add
                // the offset after returning
                chop_recursive(needle, &haystack[mid_index + 1..])
                    .map(|found_idx| found_idx + mid_index + 1)
            } else {
                // the needle can only be in the left part
                chop_recursive(needle, &haystack[..mid_index])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_chop(chop: &Fn(i32, &[i32]) -> Option<usize>) {
        assert_eq!(None, chop(3, &[]));
        assert_eq!(None, chop(3, &[1]));
        assert_eq!(Some(0), chop(1, &[1]));

        assert_eq!(Some(0), chop(1, &[1, 3, 5]));
        assert_eq!(Some(1), chop(3, &[1, 3, 5]));
        assert_eq!(Some(2), chop(5, &[1, 3, 5]));
        assert_eq!(None, chop(0, &[1, 3, 5]));
        assert_eq!(None, chop(2, &[1, 3, 5]));
        assert_eq!(None, chop(4, &[1, 3, 5]));
        assert_eq!(None, chop(6, &[1, 3, 5]));

        assert_eq!(Some(0), chop(1, &[1, 3, 5, 7]));
        assert_eq!(Some(1), chop(3, &[1, 3, 5, 7]));
        assert_eq!(Some(2), chop(5, &[1, 3, 5, 7]));
        assert_eq!(Some(3), chop(7, &[1, 3, 5, 7]));
        assert_eq!(None, chop(0, &[1, 3, 5, 7]));
        assert_eq!(None, chop(2, &[1, 3, 5, 7]));
        assert_eq!(None, chop(4, &[1, 3, 5, 7]));
        assert_eq!(None, chop(6, &[1, 3, 5, 7]));
        assert_eq!(None, chop(8, &[1, 3, 5, 7]));
    }

    #[test]
    fn test_chop_recursive() {
        test_chop(&chop_recursive);
    }
}