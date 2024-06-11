pub fn quicksort<T: PartialOrd>(slice: &mut [T]) {
    if slice.is_empty() {
        return;
    }

    let idx = partition(slice);
    let len = slice.len();

    quicksort(&mut slice[0..idx]);
    quicksort(&mut slice[idx + 1..len]);
}

fn partition<T: PartialOrd>(slice: &mut [T]) -> usize {
    if slice.len() <= 1 {
        return 0;
    }

    let mut i = 0usize;
    // use last element as pivot
    let mut pivot = slice.len() - 1;

    while i < pivot {
        // if current value is bigger than pivot, move pivot one element back
        // and swap current value to the previous position of pivot
        if slice[i] >= slice[pivot] {
            slice.swap(i, pivot - 1);
            slice.swap(pivot - 1, pivot);
            pivot -= 1;
        } else {
            i += 1;
        }
    }

    // return pivot as partition
    pivot
}

#[cfg(test)]
mod tests {
    use super::quicksort;

    fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
        for i in 1..slice.len() {
            if slice[i - 1] > slice[i] {
                return false;
            }
        }

        true
    }

    #[test]
    fn all_unsorted() {
        let mut input = [7, 6, 5, 4, 3, 2, 1];
        quicksort(&mut input);

        assert!(is_sorted(&input));
    }

    #[test]
    fn partially_sorted() {
        let mut input = [
            75, 78, 92, 1, 95, 44, 53, 43, 37, 91, 71, 18, 44, 96, 86, 6, 20, 64, 29, 24, 88, 56,
            19, 0, 32, 47, 14, 9, 86, 39, 15, 57, 71, 22, 25, 45, 71, 59, 80, 42, 6, 48, 26, 19,
            99, 6, 89, 19, 54, 18, 0, 4, 55, 0, 84, 57, 90, 67, 35, 4, 61, 22, 95, 62, 74, 55, 79,
            64, 71, 10, 53, 74, 16, 67, 54, 2, 78, 71, 99, 18, 92, 17, 95, 2, 40, 27, 28, 70, 2,
            71, 17, 94, 56, 71, 76, 66, 43, 52, 63, 71,
        ];
        quicksort(&mut input);

        assert!(is_sorted(&input));
    }

    #[test]
    fn sorted() {
        let mut input = [1, 2, 3, 4, 5, 6, 7];
        quicksort(&mut input);

        assert!(is_sorted(&input));
    }
}
