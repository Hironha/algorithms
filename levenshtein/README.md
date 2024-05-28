## Levenshtein Distance

Levenshtein distance is an algorithm to measure the distance between 2 sequences in number of operations required to transform one sequence into another. There are 3 operations available for distance calculation: **Insert**, **Delete** and **Replace**. The original algorithm uses a recursive approach to calculate the distance, but since it's really inefficient, I didn't bother to implement it.

## Matrix Algorithm

The matrix algorithm uses a matrix to calculate the distance, storing the operations required to transform every part of the sequence. It's not the most space efficient way, but the advantages of this approach is being able to tell which operations are required to achieve the equality by navigating the matrix.

Where **m** is the size of first sequence and **n** the size of the second sequence, those are the defined complexities for the matrix approach:

- Time complexity: O(m \* n)
- Space complexity: O(m \* n)

## Dynamic Algorithm

The dynamic algorithm uses 2 vectors (dynamic arrays) to calculate the distance. Instead of storing the whole matrix, it stores only the current and the previous rows, lazily allocating when necessary. The advantage of this approach is using less memory than the matrix one.

Where **m** is the size of first sequence and **n** the size of the second sequence and size of **n** >= size of **m**, those are the defined complexities:

- Time complexity: O(m \* n)
- Space complexity: O(n)
