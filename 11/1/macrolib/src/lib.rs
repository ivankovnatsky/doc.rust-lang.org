#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // Note that we’ve added a new line inside the tests module: use super::*;. The tests
    // module is a regular module that follows the usual visibility rules we covered in Chapter
    // 7 in the “Paths for Referring to an Item in the Module Tree” section. Because the tests
    // module is an inner module, we need to bring the code under test in the outer module into
    // the scope of the inner module. We use a glob here so anything we define in the outer
    // module is available to this tests module.
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larget = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Because the correct result of the can_hold function in this case is false, we need to
        // negate that result before we pass it to the assert! macro. As a result, our test will pass
        // if can_hold returns false:
        assert!(!smaller.can_hold(&larget));
    }
}
