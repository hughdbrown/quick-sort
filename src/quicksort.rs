fn quick_sort_helper<T>(values: &mut [T], lo: usize, hi: usize) 
    where T: Ord + Copy
{
    if lo < hi {
        let pivot = values[lo];
        let mut i = lo;
        let mut j = hi;
        while i < j {
            while i <= j && values[i] <= pivot {
               i += 1;
            }
            while i <= j && values[j] >= pivot {
               j -= 1;
            }
            if i < j {
                values.swap(i, j);
                //i += 1;
                //j -= 1;
            }
        }
        if j != lo {
            // println!("Swapping: {} {}", values[lo], values[j]);
            values.swap(j, lo);
            // println!("{:?}", values);
            quick_sort_helper(values, lo, j - 1);
        }
        quick_sort_helper(values, i, hi);
    }
}


pub fn quick_sort<T>(values: &mut [T])
    where T: Ord + Copy
{
    if values.len() > 1 {
        quick_sort_helper(values, 0, values.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    use crate::util::check_sorted;

    #[test]
    fn reverse_sorted() {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        quick_sort(&mut arr);
        assert!(arr == expected)

    }

    #[test]
    fn same_element() {
        let mut arr = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ];
        let expected = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ];
        quick_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn zero_elements() {
        let mut arr: [i32; 0] = [];
        let expected: [i32; 0] = [];
        quick_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn one_element() {
        let mut arr: [i32; 1] = [1];
        let expected: [i32; 1] = [1];
        quick_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn two_elements_1() {
        let mut arr: [i32; 2] = [1, 2];
        let expected: [i32; 2] = [1, 2];
        quick_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn two_elements_2() {
        let mut arr: [i32; 2] = [2, 1];
        let expected: [i32; 2] = [1, 2];
        quick_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn rand_test() {
        let mut vec: Vec<i32> = (0..100i32).collect::<Vec::<i32>>();
        let mut rng = thread_rng();
        for _ in 0..2000 {
            vec.shuffle(&mut rng);
            quick_sort(&mut vec);
            assert!(check_sorted(&vec))
        }
    }
}
