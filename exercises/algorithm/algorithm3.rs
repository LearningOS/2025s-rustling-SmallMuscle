/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]){
	//TODO
    quick_sort(array, 0, array.len()-1);
}

fn quick_sort<T: PartialOrd>(array: &mut [T], start: usize, end: usize){
    if start >= end {
        return;
    }
    let mid = partition(array, start, end);
    quick_sort(array, start, mid - 1);
    quick_sort(array, mid + 1, end);
}

fn partition<T: PartialOrd>(array: &mut [T], mut start: usize, mut end: usize) -> usize {
    let mut target = (start + end)/2;
    array.swap(target, end);
    target = end;
    end -= 1;
    while start <= end {
        if array[start] <= array[target] {
            start += 1;
        } else if array[end] > array[target] {
            end -= 1;
        } else {
            array.swap(start, end);
            start += 1;
            end -= 1;
        }
   }
   array.swap(start, target);
    start
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