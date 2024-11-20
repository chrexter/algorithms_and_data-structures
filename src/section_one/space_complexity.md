Space complexity describes the amount of memory space an algorithm uses as a function of the input size.
Similar to time complexity, it's expressed using Big O notation (e.g., O(1), O(n), O(log n)). Space complexity considers
the space used by the algorithm itself, as well as any auxiliary space used (e.g., temporary variables, data structures).

1. Here's a breakdown:

- Auxiliary Space: The extra space or temporary storage used by an algorithm during execution, beyond the input data itself.
- Input Space: The space occupied by the input data provided to the algorithm.

2. Common Space Complexities:

- O(1) - Constant Space: The algorithm uses a fixed amount of memory regardless of the input size.
- O(log n) - Logarithmic Space: The space used grows logarithmically with the input size.
- O(n) - Linear Space: The space used grows proportionally to the input size (e.g., creating an array of the same size as the input).
- O(n log n) - Log-Linear Space: Algorithms that might use a logarithmic amount of extra space for each element in the input (e.g., some sorting algorithms).
- O(n²) - Quadratic Space: The space used grows proportionally to the square of the input size (e.g. creating a 2D array where each dimension is proportional to the input).

3. Why Space Complexity Matters:

- Memory Management: Understanding space complexity helps optimize memory usage.
- Scalability: Predicting memory requirements for large datasets.
- Algorithm Selection: Choosing algorithms that fit within available memory constraints.

4. Key distinctions between Space and Time complexity:

- Focus: Time complexity deals with runtime, while space complexity deals with memory usage.
- Analysis: Both use Big O notation, but the analysis focuses on different resources.
- Trade-offs: Sometimes, there's a trade-off between time and space complexity; an algorithm that is faster might use more memory, and vice-versa.
