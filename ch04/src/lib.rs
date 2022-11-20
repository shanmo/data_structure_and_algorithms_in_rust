#![feature(test)]
extern crate test;

mod double_list;
mod single_list;
mod skip_list;

mod dynamic_array;

#[cfg(test)]
mod tests {
    use crate::*;
    use rand::thread_rng;
    use rand::Rng;
    use test::Bencher;
    const LIST_ITEMS: u64 = 15_000;

    #[bench]
    fn bench_skip_list_find(b: &mut Bencher) {
        let mut list = skip_list::BestTransactionLog::new_empty(20);
        for i in 0..LIST_ITEMS {
            list.append(i, format!("insert {}", i));
        }

        let mut rng = thread_rng();

        b.iter(|| {
            list.find(rng.gen_range::<u64>(0, LIST_ITEMS))
                .expect("not found")
        });
    }

    #[bench]
    fn bench_dynamic_array_append(b: &mut Bencher) {
        let mut arr = dynamic_array::TimestampSaver::new_empty(); 
        let mut rng = thread_rng(); 

        b.iter(|| {
            arr.append(rng.gen::<u64>())
        });
    }

    #[test]
    fn dynamic_array_append() {
        let mut arr = dynamic_array::TimestampSaver::new_empty(); 
        let max: usize = 1_000; 
        for i in 0..max {
            arr.append(i as u64); 
        }
        assert_eq!(arr.length, max);
    }

    #[test]
    fn dynamic_array_at() {
        let mut arr = dynamic_array::TimestampSaver::new_empty(); 
        let max: usize = 1_000; 
        for i in 0..max {
            arr.append(i as u64); 
        }
        assert_eq!(arr.length, max); 
        for i in 0..max {
            assert_eq!(arr.at(i), Some(i as u64));
        }
        assert_eq!(arr.at(max+1), None); 
    }

    #[test]
    fn dynamic_array_iterate() {
        let mut arr = dynamic_array::TimestampSaver::new_empty(); 
        for i in 0..5 {
            arr.append(i as u64); 
        }
        assert_eq!(arr.length, 5); 
        // let mut iter = arr.into_iter(); 
        // for i in 0..5 {
        //     assert_eq!(iter.next(), Some(i as u64)); 
        // }
        // assert_eq!(iter.next(), None); 

        let mut i: u64 = 4; 
        for a in arr.back_iter().rev() {
            assert_eq!(a, i); 
            if i > 0 {
                i -= 1; 
            } 
        }
    }
}
