#[derive(Debug, Clone, Copy, PartialEq)]
struct Element<T, U> {
    re: T,
    im: U
}

fn bubble_sort<T: Copy>(arr: &mut[T], comp: fn(&T, &T) -> bool) -> &mut[T] {
    let len: usize = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if comp(&arr[j], &arr[j + 1]) {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    return arr;
}

fn compare<T: PartialOrd, U: PartialOrd>(elem1: &Element<T, U>, elem2: &Element<T, U>) -> bool {
    if elem1.re == elem2.re {
        return elem1.im > elem2.im;
    }
    return elem1.re > elem2.re;
}

#[test]
fn test_sort_array() {
    let comp_grt = |val1: &i32, val2: &i32| -> bool { val1 > val2 };
    let comp_les = |val1: &u32, val2: &u32| -> bool { val1 < val2 };

    assert_eq!(bubble_sort(&mut [0, 1, 2, 3], comp_les),
               [3, 2, 1, 0]);
    assert_eq!(bubble_sort(&mut [0, 1, 2, 3], comp_grt),
               [0, 1, 2, 3]);
    assert_eq!(bubble_sort(&mut [5, 6, 1, 4, 3, -4, -1, 5, 0, 9], comp_grt),
               [-4, -1, 0, 1, 3, 4, 5, 5, 6, 9]);
    assert_eq!(bubble_sort(&mut [10, 9, 8, 7, 6, 5, 4, 3, 2, 1], comp_grt),
               [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_sort_elements() {
    let mut elems: [Element<f32, i32>; 4] = [
        Element {re: 11., im: 12},
        Element {re: 2., im: 12},
        Element {re: 5., im: 12},
        Element {re: 10., im: 12}
    ];

    let sorted_elems: [Element<f32, i32>; 4] = [
        Element {re: 2., im: 12},
        Element {re: 5., im: 12},
        Element {re: 10., im: 12},
        Element {re: 11., im: 12}
    ];

    assert_eq!(bubble_sort(&mut elems, compare), sorted_elems);
}

fn main() {
}
