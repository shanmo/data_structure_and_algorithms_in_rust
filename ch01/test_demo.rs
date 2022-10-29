#![feature(test)]
extern crate test; 

pub fn my_add(a: i32, b: i32) -> i32 {
    a + b 
}

#[cfg(test)]
mod tests {
    use super::*; 
    use test::Bencher; 

    #[test]
    fn this_works() {
        assert_eq!(my_add(1, 1), 2); 
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn this_does_not_work() {
        assert_eq!(my_add(std::i32::MAX, std::i32::MAX), 0); 
    }

    #[bench]
    fn how_fast(b: &mut Bencher) {
        b.iter(|| my_add(42, 42))
    }
}