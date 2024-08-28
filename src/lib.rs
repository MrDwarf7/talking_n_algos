#![allow(dead_code, unused_variables, clippy::comparison_chain)]

fn simple_binary_search(arr: &[usize], find: usize) -> Option<usize> {
    println!("\n\n-----------------Simple Binary Search-----------------");
    println!("-----------------====================-----------------\n\n");

    let length = arr.len();
    let mut half = length.div_euclid(2);
    let mut hind = length.saturating_sub(1);
    let mut lind = 0;
    let mut current = arr[half];

    while lind <= hind {
        if current == find {
            println!("Found {} at index {}", find, half);
            return Some(half);
        } else if current < find {
            println!("{} is less than {}", current, find);
            lind = half + 1;
        } else if current > find {
            println!("{} is greater than {}", current, find);
            hind = hind.saturating_sub(1);
        }
        println!("lind: {}, hind: {}", lind, hind);
        half = (hind + lind).div_euclid(2);
        current = arr[half];
        println!("half: {}, current: {}", half, current);
    }
    None
}

fn rusty_binary_search(arr: &[usize], find: usize) -> Option<usize> {
    println!("\n\n-----------------Rust Binary Search-----------------");
    println!("-----------------==================-----------------\n\n");

    let length = arr.len();
    let mut half = length.div_euclid(2);
    let mut hind = length.saturating_sub(1);
    let mut lind = 0;
    let mut current = arr[half];

    while lind <= hind {
        match current.cmp(&find) {
            std::cmp::Ordering::Equal => return Some(half),
            std::cmp::Ordering::Less => lind = half.saturating_add(1),
            std::cmp::Ordering::Greater => hind = half.saturating_sub(1),
        }
        println!("lind: {}, hind: {}", lind, hind);
        half = (hind + lind).div_euclid(2);
        println!("half: {}", half);
        println!("current: {}", arr[half]);
        current = arr[half];
    }

    None
}

#[cfg(test)]
mod test_binary_searches {
    use super::*;
    use lazy_static::lazy_static;
    use rand::Rng;
    use std::io::Write;

    const SIZE: usize = 10_000_000;

    lazy_static! {
        static ref RAND_ARR: Vec<usize> = rand::thread_rng()
            .sample_iter(&rand::distributions::Uniform::new(0, 100))
            .take(SIZE)
            .collect();
    }

    #[test]
    fn test_base_binary_search() {
        let buf = std::io::stdout();
        let mut handle = buf.lock();

        let base_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("TEST Simple find:::::: -- Finding now");

        for i in 0..10 {
            let rnd_find = 7;
            assert_eq!(
                simple_binary_search(&base_arr, rnd_find),
                rusty_binary_search(&base_arr, rnd_find)
            );
            let _ = handle.write_all(format!("Finding {} in the array\n", rnd_find).as_bytes());
            handle.flush().unwrap();
        }
    }

    #[test]
    fn test_simple_binary_search() {
        let buf = std::io::stdout();
        let mut handle = buf.lock();

        for i in 0..100 {
            let rnd_start = rand::thread_rng().gen_range(1..SIZE).to_owned();
            let rnd_find = rand::thread_rng().gen_range(rnd_start..SIZE).to_owned();
            assert_eq!(
                simple_binary_search(&RAND_ARR, rnd_find),
                rusty_binary_search(&RAND_ARR, rnd_find)
            );
            let _ = handle.write_all(format!("Finding {} in the array\n", rnd_find).as_bytes());
            handle.flush().unwrap();
        }
    }

    #[test]
    fn test_rusty_binary_search() {
        let buf = std::io::stdout();
        let mut handle = buf.lock();

        for i in 0..100 {
            let rnd_start = rand::thread_rng().gen_range(1..SIZE).to_owned();
            let rnd_find = rand::thread_rng().gen_range(rnd_start..SIZE).to_owned();
            assert_eq!(
                simple_binary_search(&RAND_ARR, rnd_find),
                rusty_binary_search(&RAND_ARR, rnd_find)
            );
            let _ = handle.write_all(format!("Finding {} in the array\n", rnd_find).as_bytes());
            handle.flush().unwrap();
        }
    }
}
