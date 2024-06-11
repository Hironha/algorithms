pub fn quicksort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
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

    let pivot_idx = slice.len() - 1;
    // `i` represents the position which pivot should be
    let mut i = 0;

    for j in 0..pivot_idx {
        if slice[j] <= slice[pivot_idx] {
            // move values smaller than pivot to left of pivot
            slice.swap(i, j);
            // increment the position pivot should be, since `i` has a value
            // smaller than pivot after the swap
            i += 1;
        }
    }

    // move pivot to the correct position
    slice.swap(i, pivot_idx);

    i
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
        let mut input = [12, 4, 3, 2, 1, 5, 4, 7, 8, 9];
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
