fn mergesort<T: Ord + Clone + std::fmt::Debug>(arr: &mut [T]) {
    println!("Sorting: {:?}", arr);
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    mergesort(&mut left);
    mergesort(&mut right);

    merge(arr, &left, &right);
    println!("After merge: {:?}", arr);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut arr_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            arr[arr_idx] = left[left_idx].clone();
            left_idx += 1;
        } else {
            arr[arr_idx] = right[right_idx].clone();
            right_idx += 1;
        }
        arr_idx += 1;
    }

    while left_idx < left.len() {
        arr[arr_idx] = left[left_idx].clone();
        left_idx += 1;
        arr_idx += 1;
    }

    while right_idx < right.len() {
        arr[arr_idx] = right[right_idx].clone();
        right_idx += 1;
        arr_idx += 1;
    }
}

fn main() {
    let mut numbers = vec![10, 7, 8, 9, 1, 5];
    println!("Before sort: {:?}", numbers);

    mergesort(&mut numbers);

    println!("After sort: {:?}", numbers);
}
