// This function returns how much icecream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no icecream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
    if hour_of_day < 22 {
        Some(5)
    } else if hour_of_day == 22 || hour_of_day == 23 {
        Some(0)
    } else {
        None
    }
    /*
    Use match for more efficient way
    match hour_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0),
        _ => None,
    }
     */
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let icecreams = maybe_icecream(12).unwrap();
        /*
        You can use pattern matching as below
        let icecreams = match maybe_icecream { Some(value) => value,
         None => {
         // Handle the None case here 0 // Or any default value you prefer
          }
         */

        assert_eq!(icecreams, 5); // Don't change this line.
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
