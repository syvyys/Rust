// task 1
fn bubble_sort(arr: &mut[i32], comp: fn(&i32, &i32)->bool) -> &mut[i32] {
    let len: usize = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if comp(&arr[j], &arr[j + 1]) {
                (arr[j], arr[j + 1]) = (arr[j + 1], arr[j]);
            }
        }
    }
    return arr;
}

#[test]
fn sort_test() {
    let comp_grt = |val1: &i32, val2: &i32| -> bool { val1 > val2 };
    let comp_les = |val1: &i32, val2: &i32| -> bool { val1 < val2 };

    assert_eq!(bubble_sort(&mut [0, 1, 2, 3], comp_les),
               [3, 2, 1, 0]);
    assert_eq!(bubble_sort(&mut [0, 1, 2, 3], comp_grt),
               [0, 1, 2, 3]);
    assert_eq!(bubble_sort(&mut [5, 6, 1, 4, 3, -4, -1, 5, 0, 9], comp_grt),
               [-4, -1, 0, 1, 3, 4, 5, 5, 6, 9]);
    assert_eq!(bubble_sort(&mut [10, 9, 8, 7, 6, 5, 4, 3, 2, 1], comp_grt),
               [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

// task 2
fn is_power_of_two(num: i32) -> bool {
    let mut num_ones: i32 = 0;
    for i in 0..32 {
        num_ones += (num >> i) & 1;
        if num_ones > 1 {
            return false;
        }
    }
    return num_ones == 1;
}

#[test]
fn power_test() {
    let base: i32 = 2;
    assert_eq!(is_power_of_two(0), false);
    assert_eq!(is_power_of_two(3), false);
    assert_eq!(is_power_of_two(456), false);
    assert_eq!(is_power_of_two(2222), false);

    assert_eq!(is_power_of_two(base.pow(0)), true);
    assert_eq!(is_power_of_two(base.pow(1)), true);
    assert_eq!(is_power_of_two(base.pow(2)), true);
    assert_eq!(is_power_of_two(base.pow(4)), true);
    assert_eq!(is_power_of_two(base.pow(5)), true);
    assert_eq!(is_power_of_two(base.pow(10)), true);
}

// task 3
fn reverse_bits(num: u32) -> u32 {
    let mut reversed: u32 = 0;
    for i in 0..32 {
        let bit = (num >> i) & 1;
        reversed = reversed | (bit << 32 - i - 1);
    }
    return reversed;
}

#[test]
fn reverse_test() {
    assert_eq!(reverse_bits(43261596), 964176192);
    assert_eq!(reverse_bits(0), 0);
    assert_eq!(reverse_bits(u32::MAX), u32::MAX);
    assert_eq!(reverse_bits(1), 2147483648);
    assert_eq!(reverse_bits(4), 536870912);
    assert_eq!(reverse_bits(98304), 98304);
}

// task 4
fn digit_sum(num: i32) -> i32 {
    let mut total = 0;
    let mut divided = num;
    while divided > 0 {
        total += divided % 10;
        divided = divided / 10;
    }
    return total;
}

fn add_digits(num: i32) -> i32 {
    let mut result = num;
    while result > 9 {
        result = digit_sum(result)
    }
    return result;
}

fn add_digits_simple(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    let remainder = num % 9;
    if remainder == 0 {
        return 9;
    }
    return remainder;
}

#[test]
fn sum_test() {
    assert_eq!(add_digits(0), 0);
    assert_eq!(add_digits(1), 1);
    assert_eq!(add_digits(10), 1);
    assert_eq!(add_digits(11111111), 8);
    assert_eq!(add_digits(9999), 9);
    assert_eq!(add_digits(15613051), 4);
    assert_eq!(add_digits(123456789), 9);
    assert_eq!(add_digits(4444), 7);
    assert_eq!(add_digits(555), 6);

    assert_eq!(add_digits_simple(0), 0);
    assert_eq!(add_digits_simple(1), 1);
    assert_eq!(add_digits_simple(10), 1);
    assert_eq!(add_digits_simple(11111111), 8);
    assert_eq!(add_digits_simple(9999), 9);
    assert_eq!(add_digits_simple(15613051), 4);
    assert_eq!(add_digits_simple(123456789), 9);
    assert_eq!(add_digits_simple(4444), 7);
    assert_eq!(add_digits_simple(555), 6);
}


// task 5
fn find_palindrome(num: i32) -> bool {
    if num < 0 {
        return false;
    }

    let mut reversed = 0;
    let mut divided = num;
    while divided > 0 {
        reversed = reversed * 10 + (divided % 10);
        divided = divided / 10;
    }
    return reversed == num;
}

#[test]
fn palindrome_test() {
    assert_eq!(find_palindrome(121), true);
    assert_eq!(find_palindrome(0), true);
    assert_eq!(find_palindrome(-50), false);
    assert_eq!(find_palindrome(155), false);
    assert_eq!(find_palindrome(123321), true);
    assert_eq!(find_palindrome(111111), true);
    assert_eq!(find_palindrome(123421), false);
    assert_eq!(find_palindrome(5554455), false);
    assert_eq!(find_palindrome(45), false);
    assert_eq!(find_palindrome(44), true);
}

fn main() {
}