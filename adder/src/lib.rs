#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }
}

#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rect{w: 8, h: 6};
        let smaller = Rect{w: 1, h: 2};

        assert!(larger.can_hold(&smaller));
    }
}
