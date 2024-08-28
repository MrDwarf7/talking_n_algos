#![allow(dead_code, unused_variables, clippy::comparison_chain)]

use std::io::Write;

fn simple_binary_search(arr: &[usize], find: usize) -> Option<usize> {
    println!("\n\n-----------------Simple Binary Search-----------------");
    println!("-----------------====================-----------------\n\n");

    let length = arr.len();
    let mut half = length.div_euclid(2);
    let mut hind = length.saturating_sub(1);
    let mut lind = 0;
    let mut current = arr[half];

    let buf = std::io::stdout();
    let mut handle = buf.lock();

    while lind <= hind {
        if current == find {
            let _ = handle.write_all(format!("Found {} at index {} \n", find, half).as_bytes());
            return Some(half);
        } else if current < find {
            let _ = handle.write_all(format!("{} is less than {}\n", current, find).as_bytes());
            lind = half + 1;
        } else if current > find {
            let _ = handle.write_all(format!("{} is greater than {}", current, find).as_bytes());
            hind = hind.saturating_sub(1);
        }

        let _ = handle.write_all(format!("lind: {}, hind: {}\n", lind, hind).as_bytes());

        half = (hind + lind).div_euclid(2);
        current = arr[half];
        let _ = handle.write_all(format!("half: {}\n", half).as_bytes());
        handle.flush().unwrap();
    }
    let _ = handle.write_all(format!("Found {} at index {}\n", find, half).as_bytes());
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

    let buf = std::io::stdout();
    let mut handle = buf.lock();

    while lind <= hind {
        match current.cmp(&find) {
            std::cmp::Ordering::Equal => return Some(half),
            std::cmp::Ordering::Less => lind = half.saturating_add(1),
            std::cmp::Ordering::Greater => hind = half.saturating_sub(1),
        }
        let _ = handle.write_all(format!("lind: {}, hind: {}\n", lind, hind).as_bytes());
        half = (hind + lind).div_euclid(2);
        let _ = handle.write_all(format!("half: {}\n", half).as_bytes());
        let _ = handle.write_all(format!("current: {}\n", arr[half]).as_bytes());
        current = arr[half];
        handle.flush().unwrap();
    }
    let _ = handle.write_all(format!("Found {} at index {}\n", find, half).as_bytes());

    None
}

#[cfg(test)]
mod test_binary_searches {
    use super::*;
    use lazy_static::lazy_static;
    use rand::Rng;
    use std::io::Write;

    const SIZE: usize = 10_000;
    const TOTAL_LOOPS: usize = 20;

    lazy_static! {
        static ref RAND_ARR: Vec<usize> = rand::thread_rng()
            .sample_iter(&rand::distributions::Uniform::new(0, 100))
            .take(SIZE)
            .collect();
    }

    #[test]
    fn test_base_binary_search() {
        const TARGET: usize = 7;
        println!("Target is: {}", TARGET);

        let buf = std::io::stdout();
        let mut handle = buf.lock();

        let base_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("TEST Simple find:::::: -- Finding now");

        for i in 0..=TOTAL_LOOPS {
            assert_eq!(
                simple_binary_search(&base_arr, TARGET),
                rusty_binary_search(&base_arr, TARGET)
            );
            let _ = handle.write_all(format!("\n\nFinding {} in the array", TARGET).as_bytes());
            handle.flush().unwrap();
        }
    }

    #[test]
    fn test_simple_binary_search() {
        const TARGET: usize = 542;
        println!("Target is: {}", TARGET);

        let buf = std::io::stdout();
        let mut handle = buf.lock();

        for i in 0..=TOTAL_LOOPS {
            let rnd_find = rand::thread_rng().gen_range(TARGET..SIZE).to_owned();
            assert_eq!(
                simple_binary_search(&RAND_ARR, rnd_find),
                rusty_binary_search(&RAND_ARR, rnd_find)
            );
            let _ = handle.write_all(format!("\n\nFinding {} in the array", rnd_find).as_bytes());
            handle.flush().unwrap();
        }
    }

    #[test]
    fn test_rusty_binary_search() {
        const TARGET: usize = 454;
        println!("Target is: {}", TARGET);

        let buf = std::io::stdout();
        let mut handle = buf.lock();

        for i in 0..=TOTAL_LOOPS {
            let rnd_find = rand::thread_rng().gen_range(TARGET..SIZE).to_owned();
            assert_eq!(
                simple_binary_search(&RAND_ARR, rnd_find),
                rusty_binary_search(&RAND_ARR, rnd_find)
            );
            let _ = handle.write_all(format!("\n\nFinding {} in the array", rnd_find).as_bytes());
            handle.flush().unwrap();
        }
    }
}
