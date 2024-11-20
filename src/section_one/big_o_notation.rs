/* Big O Definition:
 *
 * Big O notation shows how much slower an algorithm gets as its input grows.
 *
 * We say that an algorithm is O(f(n)) if the
 * number of simple operations the computer has
 * to do is eventually less than a constant
 * times f(n), as n increases.
 *
 * -> f(n) could be linear (f(n) = n)
 * -> f(n) could be quadraric (f(n) = n²)
 * -> f(n) could be constant (f(n) = 1)
 * -> f(n) could be something entirely different!
 *
 * When we talk about Big O, we talk about the worst case scenerio of an algorithm.
 *
 * 1. Constants Don't Matter:
 * - O(2n) -> O(n) : linear time
 * - O(500) -> O(1) : constant time
 * - O(13n²) -> O(n²) : quadratic time
 *
 * 2. Smaller Terms Don't Matter:
 * - O(n + 10) -> O(n)
 * - O(1000n + 50) -> O(n)
 * - O(n² + 5n + 8) -> O(n²)
 *
 * 3. Big O Shorthands
 * - Arithmetic operations are constant (+ - * %)
 * - Variable assignment is constant
 * - Accessing elements in array (by index) or object
 *   (by key) is constant.
 * - In a loop, the complexity is the length of the
 *   loop times the complexity of whatever happens
 *   inside of the loop.
*/

/*
 * Quiz 1:
 *
 * Question 1: Simplify the big O expression as much as possible - O(n + 10)
 * Solution: O(n)
 *
 * Question 2: Simplify the big O expression as much as possible - O(100 * n)
 * Solution: O(n)
 *
 * Question 3: Simplify the following big O expression as much as possible - O(25)
 * Solution: O(1)
 *
 * Question 4: Simplify the following big O expression as much as possible - O(n^2 + n^3)
 * Solution: O(n^3)
 *
 * Question 5: Simplify the following big O expression as much as possible - O(n + n + n + n)
 * Solution: O(n)
 *
*/
