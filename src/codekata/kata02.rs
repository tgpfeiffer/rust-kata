//! Implements <http://codekata.com/kata/kata02-karate-chop/>

pub fn main() {
    println!("Hello from Kata 02");
}

pub fn chop(needle: i32, haystack: &[i32]) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop() {
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
}