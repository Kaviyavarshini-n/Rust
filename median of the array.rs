fn find_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    if len % 2 == 1 {
        Some(arr[len / 2] as f64)
    } else {
        let mid_right = arr[len / 2];
        let mid_left = arr[len / 2 - 1];
        Some((mid_left as f64 + mid_right as f64) / 2.0)
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4];

    match find_median(&arr1) {
        Some(median) => println!("Median: {}", median),
        None => println!("Array is empty"),
    }

    match find_median(&arr2) {
        Some(median) => println!("Median: {}", median),
        None => println!("Array is empty"),
    }
}
