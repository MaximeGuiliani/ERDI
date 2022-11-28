use std::fs;

use std::cmp::PartialOrd;
use std::time::{Duration, Instant};

use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;

pub fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, (len - 1) as isize);
    }
}
fn main() {
    let sizes: [usize; 6]=[10000,100000,400000,750000, 1000000,2000000] ;

    let mut array: [u128; 250]=[0; 250] ;
    let mut compteur = 0;
    let mut len = 0;
    let mut orig: Vec<i32> = Uniform::from(0..100_000_000)
        .sample_iter(&mut thread_rng())
        .take(len)
        .collect();
        macro_rules! test_sort {
        ($name:expr, $sort:ident) => {
            println!(
                "Sorting {} million numbers with {} in Rust ...",
                len / 1_000_000,
                $name
            );
            let mut data = orig.clone();
            let start = Instant::now();
            data.$sort();
            let end = start.elapsed();
            array[compteur] = end.as_millis();
        }
    }

    for i in 0..6{
        len = sizes[i];

        for _j in 0..10{
            orig= Uniform::from(0..100_000_000)
            .sample_iter(&mut thread_rng())
            .take(len)
            .collect();
            test_sort!("parallel quicksort", sort);
            compteur += 1;
        }
    }
    let mut data: String = "".to_owned();


    let mut sum = 0;

    for i in 0..6{
        sum = 0;
        for j in 0..10{
            sum +=array[i*6 + j];
        }
        let mean = sum /10;
        let nb_try = sizes[i];
        let line : &str = &(nb_try.to_string() +" ," + &mean.to_string() + "\n");
        data.push_str(line);

    }
        fs::write("qs_rust.csv", data).expect("Unable to write file");

}

