use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Interval {
    pub start: i64,
    pub end: i64
}

impl From<(i64, i64)> for Interval {
    fn from(value: (i64, i64)) -> Self {
        Interval { start: i64::min(value.0, value.1), end: i64::max(value.1, value.0) }
    }
}

impl Interval {
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let start = i64::max(self.start, other.start);
        let end = i64::min(self.end, other.end);

        match start.cmp(&end) {
            Ordering::Less => Some(Self {start, end}),
            Ordering::Equal => Some(Self { start, end }),
            Ordering::Greater => None
        }
        
    }

    pub fn union(&self, other: &Self) -> Self {
        Interval { 
            start: i64::min(self.start, other.start),
            end: i64::max(self.end, other.end) 
        }
    }

    pub fn width(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
        intervals.into_iter().fold(vec![], |mut list, current_interval| {
            if list.len() == 0 {
                list.push(current_interval);
                return list;
            }
            
            let last_interval = list.pop().unwrap();
            if last_interval.intersection(&current_interval).is_some() {
                list.push(last_interval.union(&current_interval))
            }
            else {
                list.push(last_interval);
                list.push(current_interval);
            }
        
            list
        })
    }
}