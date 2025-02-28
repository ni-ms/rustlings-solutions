#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

/*
In Rust, an **iterator** is a trait that allows you to iterate over a sequence of items,
such as elements in a collection (e.g., a vector, array, or range). Iterators are a fundamental part
of Rust's design and are used extensively in idiomatic Rust code.

### The `Iterator` Trait
The core of Rust's iteration is the `Iterator` trait, which is defined in the standard library as follows:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // Provided methods (e.g., `map`, `filter`, `collect`, etc.)
}
```

- **`type Item`**: This is an associated type that represents the type of the elements being iterated over.
- **`next(&mut self) -> Option<Self::Item>`**: This is the only required method. It returns the next
item in the sequence wrapped in `Some`, or `None` if the iteration is over.

### How Iterators Work
When you call `next()` on an iterator, it advances through the sequence and returns the next item.
Once the sequence is exhausted, it returns `None`.

### Example: Using an Iterator
Here’s a simple example of using an iterator over a vector:

```rust
fn main() {
    let v = vec![1, 2, 3];

    // Get an iterator over the vector
    let mut iter = v.iter();

    // Use the iterator
    while let Some(value) = iter.next() {
        println!("{}", value);
    }
}
```

Output:
```
1
2
3
```

### Common Iterator Methods
The `Iterator` trait provides many useful methods for working with iterators. Here are a few examples:

1. **`map`**: Transforms each item in the iterator.
   ```rust
   let v = vec![1, 2, 3];
   let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
   println!("{:?}", doubled); // [2, 4, 6]
   ```

2. **`filter`**: Keeps only items that satisfy a predicate.
   ```rust
   let v = vec![1, 2, 3, 4];
   let evens: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
   println!("{:?}", evens); // [2, 4]
   ```

3. **`collect`**: Collects items into a collection (e.g., a vector).
   ```rust
   let v = vec![1, 2, 3];
   let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
   println!("{:?}", doubled); // [2, 4, 6]
   ```

4. **`for_each`**: Applies a closure to each item.
   ```rust
   let v = vec![1, 2, 3];
   v.iter().for_each(|x| println!("{}", x));
   ```

5. **`fold`**: Reduces the iterator to a single value.
   ```rust
   let v = vec![1, 2, 3];
   let sum: i32 = v.iter().fold(0, |acc, x| acc + x);
   println!("{}", sum); // 6
   ```

### Creating Custom Iterators
You can implement the `Iterator` trait for your own types. Here’s an example of a custom iterator
that counts up to a limit:

```rust
struct Counter {
    count: u32,
    limit: u32,
}

impl Counter {
    fn new(limit: u32) -> Counter {
        Counter { count: 0, limit }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.limit {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);
    for num in counter {
        println!("{}", num);
    }
}
```

Output:
```
1
2
3
4
5
```

### Lazy Evaluation
Iterators in Rust are **lazy**, meacomputer screw kitning they don’t do any work until you explicitly request the next
item (e.g., by calling `next()` or using a method like `collect()`). This makes them efficient and
composable.

### Summary
- Iterators are a powerful abstraction for working with sequences of items.
- The `Iterator` trait provides a wide range of methods for transforming and consuming iterators.
- You can create custom iterators by implementing the `Iterator` trait.
- Iterators are lazy, meaning they only compute values as needed.

Let me know if you'd like to dive deeper into any specific aspect of iterators in Rust!
 */

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if a%b == 0{
        Ok(a/b)
    } else {
        Err(DivisionError::NotDivisible)
    }
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() {
    let numbers = vec![27, 297, 38502, 81].into_iter();
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
