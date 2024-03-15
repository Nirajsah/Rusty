fn main() {
    let nums = vec![10, 2, 3, 4, 5, 6, 8];
    let res = largest(&nums);
    println!("Largest item: {res}");
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
