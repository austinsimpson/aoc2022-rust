use std::str::FromStr;

fn main() {
    let lines = include_str!("input.txt").lines();
    let pairs = lines.map(Pair::from_str)
        .filter_map(|pair| match pair {
            Ok(p) => Some(p),
            Err(_) => None
        })
        .collect::<Vec<_>>();

    let overlaps = pairs.iter().map(|pair| pair.has_overlap());
    let sum_overlaps = overlaps.fold(0, |s, o| if o { s + 1 } else { s });

    let intersections = pairs.iter().map(|pair| pair.has_intersection());
    let sum_intersections = intersections.fold(0, |s, o| if o { s + 1 } else { s });


    println!("The number of overlaps is {}, intersections is {}", sum_overlaps, sum_intersections)
}

#[derive(Clone, Copy)]
struct Section {
    start: u32,
    end: u32
}

#[derive(Clone, Copy)]
struct SectionErr;

impl FromStr for Section {
    type Err = SectionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, '-').map(|num| num.parse::<u32>().map_err(|_| SectionErr));
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        match (start, end) {
            (Ok(st), Ok(e)) => Ok(Section{start: st, end: e}),
            (_, _) => Err(SectionErr)
        }
    }
}

impl Section {
    fn overlaps_other(self, other_section: &Section) -> bool {
        self.start <= other_section.start && self.end >= other_section.end
    }

    fn intersects_other(self, other_section: &Section) -> bool {
        (other_section.start <= self.start && self.start <= other_section.end) || (other_section.start <= self.end && self.end <= other_section.end)
    }
}


#[derive(Clone, Copy)]
struct PairErr;

#[derive(Clone, Copy)]
struct Pair {
    first: Section,
    second: Section
}

impl FromStr for Pair {
    type Err = PairErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, ",").map(Section::from_str);
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        match (first, second) {
            (Ok(f), Ok(s)) => Ok(Pair{ first: f, second: s }),
            (_, _) => Err(PairErr)
        }
    }
}

impl Pair {
    fn has_overlap(self) -> bool {
        self.first.overlaps_other(&self.second) || self.second.overlaps_other(&self.first)
    }

    fn has_intersection(self) -> bool {
        self.first.intersects_other(&self.second) || self.second.intersects_other(&self.first)
    }
}