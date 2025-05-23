use std::collections::HashMap;

fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut sorted: bool = true;

    while sorted {
        sorted = false;
        for i in 0..arr.len() - 1 {
            if &arr[i] > &arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = true;
            }
        }
    }

    arr
}

fn median_index(arr: Vec<i32>) -> usize {
    arr.len() / 2
}

fn median(arr: Vec<i32>) -> i32 {
    arr[arr.len() / 2]
}

fn count(arr: Vec<i32>) -> i32 {
    let mut h = HashMap::new();

    for i in 0..arr.len() - 1 {
        let count = h.entry(arr[i]).or_insert(0);
        *count += 1;
    }

    // let mut hk: i32 = 0;
    // let mut hv: i32 = 0;

    // for (k, v) in h.iter() {
    //     if v > &hv {
    //         hv = *v;
    //         hk = *k
    //     }
    // }

    h.into_iter()
        .max_by_key(|&(_, v)| v)
        .map(|(k, _)| k)
        .unwrap()

    // hk
}

fn main() {
    let m = vec![-1, 56, 0, -1, 100];

    // true
    // [-1, 56, 0, -1, 100];
    //   false
    //   [-1, 56, 0, -1, 100]
    //   false
    //   [-1, 0, 56, -1, 100]
    //   true
    //   [-1, 0, -1, 56, 100]
    //   true
    //   [-1, 0, -1, 56, 100]
    //   true
    // [-1, 0, -1, 56, 100]
    // false
    //   [-1, 0, -1, 56, 100]
    //   false
    //   [-1, -1, 0, 56, 100]
    //   true
    //   [-1, -1, 0, 56, 100]
    //   true
    //   [-1, -1, 0, 56, 100]
    //   true
    //   [-1, -1, 0, 56, 100]
    //   true
    // [-1, -1, 0, 56, 100]
    // false
    //   [-1, -1, 0, 56, 100]
    //   [-1, -1, 0, 56, 100]
    //   [-1, -1, 0, 56, 100]
    //   [-1, -1, 0, 56, 100]
    //   [-1, -1, 0, 56, 100]
    //   false
    // false
    // loop exit

    println!("{:?}", bubble_sort(m.clone()));
    println!("{:?}", median_index(bubble_sort(m.clone())));
    println!("{:?}", median(bubble_sort(m.clone())));
    println!("{:?}", count(m));
}
