pub fn solve_a(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let mut iter = line.split(',');
            let interval1 = Interval::from(iter.next().unwrap());
            let interval2 = Interval::from(iter.next().unwrap());

            i32::from(interval1.contains(&interval2))
        })
        .sum()
}

pub fn solve_b(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let mut iter = line.split(',');
            let interval1 = Interval::from(iter.next().unwrap());
            let interval2 = Interval::from(iter.next().unwrap());

            i32::from(interval1.overlaps(&interval2))
        })
        .sum()
}

struct Interval {
    pub left: i32,
    pub right: i32,
}

impl Interval {
    pub fn contains(&self, other: &Interval) -> bool {
        (self.left <= other.left && self.right >= other.right)
            || (other.left <= self.left && other.right >= self.right)
    }

    pub fn overlaps(&self, other: &Interval) -> bool {
        i32::max(self.left, other.left) <= i32::min(self.right, other.right)
    }
}

// Yes, this should be ``TryFrom`` and properly handle errors.
impl<'a> From<&'a str> for Interval {
    fn from(val: &'a str) -> Self {
        let mut iter = val.split("-");

        let left = iter.next().unwrap().parse::<i32>().unwrap();
        let right = iter.next().unwrap().parse::<i32>().unwrap();

        Self { left, right }
    }
}
