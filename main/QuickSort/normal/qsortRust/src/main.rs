use rayon::join;
use std::fs;

fn partition<T, F>(d: &mut [T], is_less: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    d.swap(0, d.len() / 2);
    let mut mid = 0;
    for i in 1..d.len() {
        if is_less(&d[i], &d[0]) {
            mid += 1;
            d.swap(i, mid);
        }
    }
    d.swap(0, mid);
    mid
}

fn insert_sort<T, F>(d: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..d.len() {
        let mut n = i;
        while n > 0 && is_less(&d[n], &d[n - 1]) {
            d.swap(n, n - 1);
            n -= 1;
        }
    }
}

fn quick_sort<T, F>(d: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool,
{
    if d.len() > 30 {
        let mut mid = partition(d, is_less);
        if mid < d.len() / 2 {
            mid += 1;
        }
        let (left, right) = d.split_at_mut(mid);
        quick_sort(left, is_less);
        quick_sort(right, is_less);
    } else {
        insert_sort(d, is_less);
    }
}

fn par_quick_sort<T, F>(d: &mut [T], is_less: &F)
where
    F: Fn(&T, &T) -> bool + Send + Sync,
    T: Send,
{
    if d.len() > 30 {
        let mut mid = partition(d, is_less);
        if mid < d.len() / 2 {
            mid += 1;
        }
        let (left, right) = d.split_at_mut(mid);

        if right.len() > 100_000 {
            join(
                || par_quick_sort(left, is_less),
                || par_quick_sort(right, is_less),
            );
        } else {
            quick_sort(left, is_less);
            quick_sort(right, is_less);
        }
    } else {
        insert_sort(d, is_less);
    }
}

pub trait QSort<T> {
    fn qsort(&mut self);

    fn is_sorted(&self) -> bool;
}

impl<T> QSort<T> for [T]
where
    T: Ord,
{
    fn qsort(&mut self) {
        quick_sort(self, &|a: &T, b: &T| a.lt(b));
    }

    fn is_sorted(&self) -> bool {
        self.windows(2).all(|w| w[0] <= w[1])
    }
}

pub trait ParQSort<T> {
    fn par_qsort(&mut self);
}

impl<T> ParQSort<T> for [T]
where
    T: Ord + Send,
{
    fn par_qsort(&mut self) {
        par_quick_sort(self, &|a: &T, b: &T| a.lt(b));
    }
}

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::time::Instant;

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
            test_sort!("parallel quicksort", qsort);
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

