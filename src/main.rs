fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);

    let mut arr_str = ["banana", "apple", "orange", "grape"];
    println!("Original string array: {:?}", arr_str);
    bubble_sort(&mut arr_str);
    println!("Sorted string array: {:?}", arr_str);
}
