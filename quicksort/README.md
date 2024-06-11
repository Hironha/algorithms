## Quicksort

Quicksort is one of the most efficient sort algorithms that uses a divide and conquer strategy to improve the average sorting speed. It also does not utilize auxiliary space, except for the stack frames used in recursion, and it's ordinary algorithm is quite easy to implement, although it can get some edge cases where it gets off by one if the partition is not done correctly.

The most important part of this algorithm is the `partition`. The partition is responsible for sorting parts of the array. The partition function selects a `pivot`, generally the last element of the slice, and makes sure every element before of the pivot is smaller, meaning the pivot it's in the right place.

- Time Complexity: O(n²) but on average O(n \* log(n))
- Space Complexity: O(1) ignoring stack frames used by recursion
