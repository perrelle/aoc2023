use std::ops::{Add,Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T> (pub T, pub T);

impl<T> Interval<T> {
    pub fn cardinal(&self) -> T
            where T: Copy + Sub<T, Output=T> + Add<T, Output=T> + From<u8> {
        self.1 - self.0 + T::from(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitResult { Below, Above }

impl<T> Interval<T> {
    pub fn split_before(&self, x: T) -> Vec<(SplitResult,Self)>
            where  T: Copy + Ord + Sub<T, Output=T> + From<u8> {
        if self.0 >= x {
            vec![(SplitResult::Above,*self)]
        } else if self.1 < x {
            vec![(SplitResult::Below,*self)]
        } else {
            vec![
                (SplitResult::Below, Interval(self.0, x - T::from(1))),
                (SplitResult::Above, Interval(x, self.1))]
        }
    }

    pub fn split_after(&self, x: T) -> Vec<(SplitResult, Self)>
            where  T: Copy + Ord + Add<T, Output=T> + From<u8> {
        if self.0 > x {
            vec![(SplitResult::Above,*self)]
        } else if self.1 <= x {
            vec![(SplitResult::Below,*self)]
        } else {
            vec![
                (SplitResult::Below, Interval(self.0, x)),
                (SplitResult::Above, Interval(x + T::from(1), self.1))]
        }
    }

}