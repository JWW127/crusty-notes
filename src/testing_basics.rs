#![allow(dead_code)]
//simple add function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//will use this later for bool assertion ----------------------
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//-------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // passes for obvios reasons
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // checks equality
    }

    #[test]
    fn it_works_2() {
        let result = add(2, 2);
        assert_ne!(result, 5); // checks equality
    }

    #[test] // this fails because panic
    fn it_fails() {
        panic!("this makes it fail")
    }

    #[test] // this fails and shows how to add custom messages
    fn it_fails_2() {
        let result = false;
        assert!(result, "\n🤬{}🤬\n", result)
    }

    #[test] //check if small rect fits in big rect
    fn rect_fits_in_rect() {
        let larger_rect = Rect {
            width: 10,
            height: 20,
        };
        let smaller_rect = Rect {
            width: 5,
            height: 10,
        };

        assert!(larger_rect.can_hold(&smaller_rect)) // checks bool truth
    }

    #[test] // checks if big rect doesnt fit in small rect
    fn rect_does_not_fits_in_rect() {
        let larger_rect = Rect {
            width: 10,
            height: 20,
        };
        let smaller_rect = Rect {
            width: 5,
            height: 10,
        };

        assert!(!smaller_rect.can_hold(&larger_rect)) // checks bool truth
    }

    // a way to check for failure
    #[test]
    #[should_panic(expected = "🤖rect to not fit but it does🤖")] // will pass if test fails/panics
    fn does_not_fit_should_panic() {
        let larger_rect = Rect {
            width: 10,
            height: 20,
        };
        let smaller_rect = Rect {
            width: 5,
            height: 10,
        };

        assert!(smaller_rect.can_hold(&larger_rect)) // checks bool truth
                                                     // Note that #[should_panic] is it can pass test from unexpected panics,
                                                     // this can used to narrow causes
    }

    #[test]
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err("☠☠☠ two plus two must equal 4 ☠☠☠".to_string())
        }
    }

    #[test] // this fails because panic
    #[ignore] // this prevents test from running
    fn it_fails_ignored() {
        panic!("this makes it fail")
    } // ignoring a test is a good idea for expensive test
}
