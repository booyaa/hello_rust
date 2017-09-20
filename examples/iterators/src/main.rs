fn main() {
    let a = [1, 2, 3];

    let (even, odd): (Vec<i32>, Vec<i32>) = a.into_iter()
                      .partition(|&n| n % 2 == 0);

    assert_eq!(even, vec![2]);
    assert_eq!(odd, vec![1, 3]);
}
