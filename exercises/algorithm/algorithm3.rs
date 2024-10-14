/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd + Copy + std::fmt::Display>(array: &mut [T]){
    let len = array.len();
    for i in 1..len {
        let x = array[i];
        let mut j = i - 1;
        while j >= 0 && x < array[j] {
            array[j + 1] = array[j];
            if (j == 0) {
                break;
            }
            j -= 1;
        }
        if (j == 0 && x < array[0]) {
            array[j] = x;

        }
        else if (x != array[j + 1]) {
            array[j + 1] = x;

        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}