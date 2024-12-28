fn quicksort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    println!("Sorting: {:?}", arr);

    let pivot_index = partition(arr);
    println!(
        "Partitioned: {:?} | Pivot element: {:?}",
        arr, arr[pivot_index]
    );

    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..len]);

    println!("Sorted section: {:?}", arr);
}

fn partition<T: Ord + std::fmt::Debug>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn main() {
    let mut numbers = vec![10, 7, 8, 9, 1, 5];
    println!("Before sorting: {:?}", numbers);

    quicksort(&mut numbers);

    println!("After sorting: {:?}", numbers);
}
