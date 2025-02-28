// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        /*
        The .next returns the next item wrapped in an Option

        You can use if let to destructure the some
        if let Some(fruit) = fav_fruits_iterator.next() {
            assert_eq!(fruit, "banana");
        }

        Or use match to destructure it..

        match fav_fruits_iterator.next() {
            Some(fruit) => assert_eq!(fruit, "banana"),
            None => panic!("No more fruits!"),
        }

         */
        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None); // TODO: Replace `todo!()`
    }
}
