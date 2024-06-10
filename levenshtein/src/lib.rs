// code reference: https://www.geeksforgeeks.org/introduction-to-levenshtein-distance/

// uses dynamic programming to avoid allocating matrix
#[must_use]
pub fn levenshtein_dynamic(left: &str, right: &str) -> usize {
    if left.is_empty() || right.is_empty() {
        return left.len().max(right.len());
    }

    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let m = left.len() + 1;
    let n = right.len() + 1;
    // initialize first row of the levenshtein matrix
    let mut prev: Vec<usize> = (0..n).collect();
    let mut curr = vec![0usize; n];

    for i in 1..m {
        // same as defining the first column in the matrix example, but does it lazily
        curr[0] = i;

        for j in 1..n {
            if left[i - 1] == right[j - 1] {
                // get operations of previous solution
                curr[j] = prev[j - 1];
            } else {
                // get min operations required to substring equality
                let min = curr[j - 1].min(prev[j]).min(prev[j - 1]);
                curr[j] = min + 1;
            }
        }

        prev.clone_from(&curr);
    }

    curr[n - 1]
}

// uses a matrix to calculate distance
pub fn levenshtein_matrix(left: &str, right: &str) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let rows = left.len() + 1;
    let cols = right.len() + 1;
    let mut matrix = vec![vec![0usize; cols]; rows];

    // initialize first column
    for (i, row) in matrix.iter_mut().enumerate() {
        row[0] = i;
    }

    // initialize first row
    for (i, col) in matrix[0].iter_mut().enumerate() {
        *col = i;
    }

    for i in 1..rows {
        for j in 1..cols {
            if left[i - 1] == right[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                let min = matrix[i][j - 1]
                    .min(matrix[i - 1][j])
                    .min(matrix[i - 1][j - 1]);

                matrix[i][j] = min + 1;
            }
        }
    }

    matrix[left.len()][right.len()]
}

#[cfg(test)]
mod tests {
    use super::{levenshtein_dynamic, levenshtein_matrix};

    #[test]
    fn dynamic() {
        let left = "hiro";
        let right = "hironha";
        assert_eq!(levenshtein_dynamic(left, right), 3);

        let left = "teste";
        let right = "test";
        assert_eq!(levenshtein_dynamic(left, right), 1);

        let left = "assert";
        let right = "asert";
        assert_eq!(levenshtein_dynamic(left, right), 1);

        let left = "schwarzenegger";
        let right = "schawarnagger";
        assert_eq!(levenshtein_dynamic(left, right), 4);
    }

    #[test]
    fn matrxi() {
        let left = "hiro";
        let right = "hironha";
        assert_eq!(levenshtein_matrix(left, right), 3);

        let left = "teste";
        let right = "test";
        assert_eq!(levenshtein_matrix(left, right), 1);

        let left = "assert";
        let right = "asert";
        assert_eq!(levenshtein_matrix(left, right), 1);

        let left = "schwarzenegger";
        let right = "schawarnagger";
        assert_eq!(levenshtein_matrix(left, right), 4);
    }
}
