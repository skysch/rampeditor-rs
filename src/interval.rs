// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a basic bounded interval type for doing complex set selections.
//!
////////////////////////////////////////////////////////////////////////////////
use std::ops::{Deref, Sub};
use std::cmp::Ord;

////////////////////////////////////////////////////////////////////////////////
// Boundary
////////////////////////////////////////////////////////////////////////////////
///
/// Determines the type of an interval's boundary.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Boundary<T> where T: PartialOrd + PartialEq + Clone {
    /// The boundary includes the point.
    Include(T),
    /// The boundary excludes the point.
    Exclude(T),
}

impl<T> Boundary<T> where T: PartialOrd + PartialEq + Clone {
    /// Returns whether the boundary includes its point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Boundary;
    ///
    /// let b1 = Boundary::Include(0);
    /// let b2 = Boundary::Exclude(1);
    /// 
    /// assert!(b1.is_closed());
    /// assert!(!b2.is_closed());
    /// ```
    #[inline]
    pub fn is_closed(&self) -> bool {
        match self {
            &Boundary::Include(..) => true,
            &Boundary::Exclude(..) => false
        }
    }

    /// Returns whether the boundary excludes its point. 
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Boundary;
    ///
    /// let b1 = Boundary::Include(0);
    /// let b2 = Boundary::Exclude(1);
    /// 
    /// assert!(!b1.is_open());
    /// assert!(b2.is_open());
    /// ```
    #[inline]
    pub fn is_open(&self) -> bool {
        !self.is_closed()
    }

    /// Returns the intersect of the given boundaries, or the lowest one if they
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Boundary;
    ///
    /// let b1 = Boundary::Include(0);
    /// let b2 = Boundary::Exclude(0);
    /// 
    /// assert_eq!(b1.intersect_or_least(&b2), b2);
    /// ```
    pub fn intersect_or_least(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_closed() && other.is_closed() {
                self.clone()
            } else {
                Boundary::Exclude((**self).clone())
            }
        } else if **self < **other {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the intersect of the given boundaries, or the greatest one if 
    /// they are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Boundary;
    ///
    /// let b1 = Boundary::Include(0);
    /// let b2 = Boundary::Exclude(0);
    /// 
    /// assert_eq!(b1.intersect_or_greatest(&b2), b2);
    /// ```
    pub fn intersect_or_greatest(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_closed() && other.is_closed() {
                self.clone()
            } else {
                Boundary::Exclude((**self).clone())
            }
        } else if **self > **other {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the union of the given boundaries, or the lowest one if they are
    /// not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Boundary;
    ///
    /// let b1 = Boundary::Include(0);
    /// let b2 = Boundary::Exclude(0);
    /// 
    /// assert_eq!(b1.union_or_least(&b2), b1);
    /// ```
    pub fn union_or_least(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_open() && other.is_open() {
                self.clone()
            } else {
                Boundary::Include((**self).clone())
            }
        } else if **self < **other {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the union of the given boundaries, or the greatest one if they 
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Boundary;
    ///
    /// let b1 = Boundary::Include(0);
    /// let b2 = Boundary::Exclude(0);
    /// 
    /// assert_eq!(b1.union_or_greatest(&b2), b1);
    /// ```
    pub fn union_or_greatest(&self, other: &Self) -> Self {
        if **self == **other {
            if self.is_open() && other.is_open() {
                self.clone()
            } else {
                Boundary::Include((**self).clone())
            }
        } else if **self > **other {
            self.clone()
        } else {
            other.clone()
        }
    }
}

// Implemented to prevent having to match on the Boundary enum to use its 
// contents.
impl<T> Deref for Boundary<T> where T: PartialOrd + PartialEq + Clone {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match *self {
            Boundary::Include(ref bound) => bound,
            Boundary::Exclude(ref bound) => bound
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Interval<T>
////////////////////////////////////////////////////////////////////////////////
///
/// A contiguous range of the type T, which may include or exclude either 
/// boundary.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Interval<T> where T: PartialOrd + PartialEq + Clone {
    /// The start of the range.
    start: Boundary<T>,
    /// The end of the range.
    end: Boundary<T>
}

impl <T> Interval<T> where T: PartialOrd + PartialEq + Clone  {
    /// Creates a new interval from the given boundaries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Boundary, Interval};
    ///
    /// let l = Boundary::Include(12);
    /// let r = Boundary::Include(16);
    /// let int = Interval::new(l, Some(r));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
    /// ```
    ///
    /// If the arguments are out of order, they will be swapped:
    ///
    /// ```rust
    /// use rampeditor::{Boundary, Interval};
    ///
    /// let l = Boundary::Include(12);
    /// let r = Boundary::Include(16);
    /// let int = Interval::new(r, Some(l));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
    /// ```
    pub fn new(start: Boundary<T>, end: Option<Boundary<T>>) -> Self {
        if let Some(end_bound) = end {
            if *end_bound < *start {
                Interval {start: end_bound, end: start}
            } else {
                Interval {start: start, end: end_bound}
            }
        } else {
            Interval {start: start.clone(), end: start}
        }
    }

    /// Creates a new open interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_closed());
    /// ```
    pub fn open(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Exclude(start),
            Some(Boundary::Exclude(end))
        )
    }

    /// Creates a new closed interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::closed(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_closed());
    /// ```
    pub fn closed(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Include(start),
            Some(Boundary::Include(end))
        )
    }

    /// Creates a new left-open interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::left_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_closed());
    /// ```
    pub fn left_open(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Exclude(start),
            Some(Boundary::Include(end))
        )
    }

    /// Creates a new right-open interval from the given values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_closed());
    /// ```
    pub fn right_open(start: T, end: T) -> Self {
        Interval::new(
            Boundary::Include(start),
            Some(Boundary::Exclude(end))
        )
    }

    /// Returns the leftmost (least) boundary point of the interval. Note that 
    /// this point may not be in the interval if the interval is left-open.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// ```
    #[inline]
    pub fn left_point(&self) -> T {
        (*self.start).clone()
    }

    /// Returns the rightmost (greatest) boundary point of the interval. Note 
    /// that this point may not be in the interval if the interval is 
    /// right-open.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.right_point(), 2);
    /// ```
    #[inline]
    pub fn right_point(&self) -> T {
        (*self.end).clone()
    }

    /// Returns the left (least) boundary of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_bound(), Boundary::Exclude(0));
    /// ```
    #[inline]
    pub fn left_bound(&self) -> Boundary<T> {
        self.start.clone()
    }

    /// Returns the right (greatest) boundary of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.right_bound(), Boundary::Exclude(2));
    /// ```
    #[inline]
    pub fn right_bound(&self) -> Boundary<T> {
        self.end.clone()
    }

    /// Returns whether a given interval is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    /// let int = Interval::right_open(0, 2);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// An open interval with two of the same points is empty:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int = Interval::open(0, 0);
    /// assert!(int.is_empty());
    /// ```
    ///
    /// A half-open interval with two of the same points is not:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int = Interval::left_open(0, 0);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// A single-point interval is empty only if that point is excluded:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int_a = Interval::new(Boundary::Exclude(0), None);
    /// let int_b = Interval::new(Boundary::Include(0), None);
    /// assert!(int_a.is_empty());
    /// assert!(!int_b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.left_bound() == self.right_bound() 
            && self.left_bound().is_open()
    }

    /// Returns whether the given point is included in the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    /// let int = Interval::right_open(0.0, 2.0);
    /// assert!(int.contains(&0.0));
    /// assert!(int.contains(&1.0));
    /// assert!(!int.contains(&2.0));
    /// ```
    #[inline]
    pub fn contains(&self, point: &T) -> bool {
        *point > self.left_point() && *point < self.right_point()
            || *point == self.left_point() && self.left_bound().is_closed()
            || *point == self.right_point() && self.right_bound().is_closed()
    }

    /// Returns the set union of the interval with the given interval. Note that
    /// since an interval requires contiguous points, a union of disjoint 
    /// intervals will fail to produce an interval and None will be returned.
    pub fn union(&self, other: &Self) -> Option<Self> {
        unimplemented!()
    }

    /// Returns the set intersection of the interval with the given interval,
    /// or None if the intervals do not overlap.
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        // Check if either one is empty.
        if self.is_empty() || other.is_empty() {
            return None;
        }

        // Check if they're the same set.
        if self == other {
            return Some(self.clone());
        }

        // a:[], b:{}
        let (a, b) = if self.left_point() <= other.left_point() {
            (self, other)
        } else {
            (other, self)
        };

        
        if a.right_point() < b.left_point() {
            // []_{}    -> None
            // [_]_{}   -> None
            // []_{_}   -> None
            // [_]_{_}  -> None
            None
        } else if a.right_point() == b.left_point() {
            // [_]{_}   -> ]{ or None
            if a.right_bound().is_closed() && b.left_bound().is_closed() {
                Some(Interval::new(
                    Boundary::Include(a.right_point()), 
                    None
                ))
            } else {
                None
            }
        } else {
            // [_{_]_}
            Some(Interval::new(
                 a.left_bound().intersect_or_greatest(&b.left_bound()),
                 Some(a.right_bound().intersect_or_least(&b.right_bound()))
            ))
        }
    }

    /// Returns the interval with all the points in the intersection with the 
    /// given interval removed.
    pub fn minus(&self, other: &Self) -> Option<Self> {
        unimplemented!()
    }

    /// Returns the smallest interval containing both of the given intervals.
    pub fn connect(&self, other: &Self) -> Option<Self> {
        unimplemented!()
    }

    /// Transforms a collection of intervals by combining any intervals that 
    /// overlap or touch and removing any that are empty.
    pub fn normalize(intervals: Vec<Self>) -> Vec<Self> {
        unimplemented!()
    }
}

impl <'a, T> Interval<T> 
    where 
        T: PartialOrd + PartialEq + Clone + 'a, 
        &'a T: Sub  
{
    /// Returns the width of the interval.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rampeditor::{Interval, Boundary};
    /// let int = Interval::open(0.0, 2.2);
    ///
    /// assert_eq!(int.width(), 2.2);
    /// ```
    ///
    /// If the interval is empty, a default value is returned:
    ///
    /// ```rust
    /// # use rampeditor::{Interval, Boundary};
    /// let int = Interval::open(0.0, 0.0);
    ///
    /// assert_eq!(int.width(), 0.0);
    /// ```
    pub fn width(&'a self) -> <&'a T as Sub>::Output 
        where <&'a T as Sub>::Output: Default 
    {
        &*self.end - &*self.start
    }
}



////////////////////////////////////////////////////////////////////////////////
// Test Module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::{Boundary, Interval};

    /// Tests the Interval::intersect function.
    #[test]
    fn interval_contains() {
        let int = Interval::right_open(0.0, 2.0);
        assert!(!int.contains(&-1.34));
        assert!(!int.contains(&-0.001));
        assert!(int.contains(&0.0));
        assert!(int.contains(&0.001));
        assert!(int.contains(&1.0));
        assert!(int.contains(&1.9999));
        assert!(!int.contains(&2.0));
    }

    /// Tests the Interval::intersect function.
    #[test]
    fn interval_intersect() {
        let o: fn(f32, f32) -> Interval<f32> = Interval::open;
        let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
        let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
        let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

        // Open overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.0, 2.0)), Some( o(1.0, 2.0)));

        // Closed overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 2.0)), Some(lo(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 2.0)), Some(ro(1.0, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 2.0)), Some( c(1.0, 2.0)));
        
        // Open left-half overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.0, 1.5)), Some( o(1.0, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.0, 1.5)), Some(lo(1.0, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.0, 1.5)), Some( o(1.0, 1.5)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.0, 1.5)), Some(lo(1.0, 1.5)));

        // Close left-half overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 1.5)), Some( o(1.0, 1.5)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 1.5)), Some(lo(1.0, 1.5)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 1.5)), Some(ro(1.0, 1.5)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 1.5)), Some( c(1.0, 1.5)));

        // Open right-half overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.5, 2.0)), Some( o(1.5, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.5, 2.0)), Some( o(1.5, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.5, 2.0)), Some(ro(1.5, 2.0)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.5, 2.0)), Some(ro(1.5, 2.0)));

        // Closed right-half overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.5, 2.0)), Some( o(1.5, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.5, 2.0)), Some(lo(1.5, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.5, 2.0)), Some(ro(1.5, 2.0)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.5, 2.0)), Some( c(1.5, 2.0)));

        // Open Subset overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(1.2, 1.8)), Some( o(1.2, 1.8)));
        assert_eq!( o(1.0, 2.0).intersect(&lo(1.2, 1.8)), Some(lo(1.2, 1.8)));
        assert_eq!( o(1.0, 2.0).intersect(&ro(1.2, 1.8)), Some(ro(1.2, 1.8)));
        assert_eq!( o(1.0, 2.0).intersect(& c(1.2, 1.8)), Some( c(1.2, 1.8)));

        // Closed Subset overlapping.
        assert_eq!( c(1.0, 2.0).intersect(& o(1.2, 1.8)), Some( o(1.2, 1.8)));
        assert_eq!( c(1.0, 2.0).intersect(&lo(1.2, 1.8)), Some(lo(1.2, 1.8)));
        assert_eq!( c(1.0, 2.0).intersect(&ro(1.2, 1.8)), Some(ro(1.2, 1.8)));
        assert_eq!( c(1.0, 2.0).intersect(& c(1.2, 1.8)), Some( c(1.2, 1.8)));

        // Right non-overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&lo(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&ro(2.0, 3.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(& c(2.0, 3.0)), None);

        // Left non-overlapping.
        assert_eq!( o(1.0, 2.0).intersect(& o(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&lo(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(&ro(0.0, 1.0)), None);
        assert_eq!( o(1.0, 2.0).intersect(& c(0.0, 1.0)), None);

        // Point intersections.
        assert_eq!( o(1.0, 2.0).intersect(& o(0.5, 0.5)), None);
        // assert_eq!( o(1.0, 2.0).intersect(&lo(0.5, 0.5)), Some( c(0.5, 0.5)));
        // assert_eq!( o(1.0, 2.0).intersect(&ro(0.5, 0.5)), Some( c(0.5, 0.5)));
        // assert_eq!( o(1.0, 2.0).intersect(& c(0.5, 0.5)), Some( c(0.5, 0.5)));
    }
}