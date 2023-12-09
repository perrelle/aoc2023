use std::ops;

#[derive (Debug, Clone, Copy)]
struct Interval(i64, i64);

impl ops::Add<i64> for &Interval {
    type Output = Interval;

    fn add(self, x: i64) -> Interval {
        Interval(self.0 + x, self.1 + x)
    }
}

impl Interval {
    fn is_empty(&self) -> bool {
        self.0 > self.1
    }

    fn wrap_empty(&self) -> Option<Self> {
        if self.is_empty() {
            None
        }
        else {
            Some (*self)
        }
    }
}

#[derive (Debug, Clone)]
pub struct Range {
    destination_start: i64,
    source_start: i64,
    length: i64
}

impl Range {
    fn source(&self) -> Interval {
        Interval(
            self.source_start,
            self.source_start + self.length - 1
        )
    }

    fn destination(&self) -> Interval {
        Interval(
            self.destination_start,
            self.destination_start + self.length - 1
        )
    }

    fn map(&self, x: i64) -> Option<i64> {
        if x >= self.source_start && x < self.source_start + self.length {
            Some (x + self.destination_start - self.source_start)
        }
        else {
            None
        }
    }

    fn map_interval(&self, interval: &Interval)
            -> (Option<Interval>, Vec<Interval>) {
        let offset_interval =
            interval + (self.destination_start - self.source_start);
        let src = self.source();
        let dst = self.destination();
        let diff_left = Interval(
            interval.0,
            interval.1.min(src.0 - 1));
        let inter = Interval(
            offset_interval.0.max(dst.0),
            offset_interval.1.min(dst.1));
        let diff_right = Interval(
            interval.0.max(src.1 + 1), 
            interval.1);

        let diffs = vec![diff_left.wrap_empty(), diff_right.wrap_empty()];
        let unwrapped_diffs : Vec<Interval> =
            diffs.into_iter().flatten().collect();
        (inter.wrap_empty(), unwrapped_diffs)
    }
}

#[derive (Debug, Clone)]
struct Ranges (Vec<Range>);

impl Ranges {
    fn map(&self, x: i64) -> i64 {
        for range in &self.0 {
            if let Some(y) = range.map(x) {
                return y
            }
        }
        x
    }

    fn map_vector(&self, ids: Vec<i64>) -> Vec<i64> {
        ids.iter().map(|id| self.map(*id)).collect()
    }

    fn map_intervals(&self, intervals: &[Interval]) -> Vec<Interval> {
        let mut result : Vec<Interval> = Vec::new();
        let mut rest : Vec<Interval> = Vec::from(intervals);

        for range in &self.0 {
            let mut next_rest : Vec<Interval> = Vec::new();

            for itv in rest {
                let (inter, diffs) = range.map_interval(&itv);
                if let Some(itv) = inter {
                    result.push(itv);
                }
                next_rest.extend(diffs);
            }

            rest = next_rest;
        }

        result.extend(rest);
        result
    }
}

#[derive (Debug, Clone)]
pub struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Ranges,
    soil_to_fertilizer: Ranges,
    fertilizer_to_water: Ranges,
    water_to_light: Ranges,
    light_to_temperature: Ranges,
    temperature_to_humidity: Ranges,
    humidity_to_location: Ranges,
}


mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        bytes::complete::*,
        multi::*
    };

    use super::*;

    fn i64(input: &str) -> IResult<&str, i64> {
        map(u32, |x| i64::try_from(x).unwrap())(input)
    }

    fn range(input: &str) -> IResult<&str, Range> {
        map(
                tuple((i64, space1, i64, space1, i64)),
                |(x, _, y, _, z)| Range {
                    destination_start: x,
                    source_start: y,
                    length: z  }
            )
            (input)
    }

    fn ranges(input: &str) -> IResult<&str, Ranges> {
        map(separated_list1(multispace1, range), Ranges)(input)
    }

    fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
        separated_list1(space1, i64)(input)
    }

    fn almanac(input: &str) -> IResult<&str, Almanac> {
        let (input, (
                seeds,_,
                seed_to_soil,_,
                soil_to_fertilizer,_,
                fertilizer_to_water,_,
                water_to_light,_,
                light_to_temperature,_,
                temperature_to_humidity,_,
                humidity_to_location)) =
            tuple((
                    preceded(pair(tag("seeds:"), multispace0), seeds),
                    multispace1,
                    preceded(pair(tag("seed-to-soil map:"), multispace1), ranges),
                    multispace1,
                    preceded(pair(tag("soil-to-fertilizer map:"), multispace1), ranges),
                    multispace1,
                    preceded(pair(tag("fertilizer-to-water map:"), multispace1), ranges),
                    multispace1,
                    preceded(pair(tag("water-to-light map:"), multispace1), ranges),
                    multispace1,
                    preceded(pair(tag("light-to-temperature map:"), multispace1), ranges),
                    multispace1,
                    preceded(pair(tag("temperature-to-humidity map:"), multispace1), ranges),
                    multispace1,
                    preceded(pair(tag("humidity-to-location map:"), multispace1), ranges)
            ))(input)?;
        let almanac = Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
         };
         Ok ((input, almanac))
    }

    pub fn parse(input: &str) -> IResult<&str, Almanac> {
        all_consuming(terminated(almanac, multispace0))(input)
    }
}


pub fn solve_part1(almanac: &Almanac) -> i64 {
    let ids = almanac.seeds.clone();
    let ids = almanac.seed_to_soil.map_vector(ids);
    let ids = almanac.soil_to_fertilizer.map_vector(ids);
    let ids = almanac.fertilizer_to_water.map_vector(ids);
    let ids = almanac.water_to_light.map_vector(ids);
    let ids = almanac.light_to_temperature.map_vector(ids);
    let ids = almanac.temperature_to_humidity.map_vector(ids);
    let ids = almanac.humidity_to_location.map_vector(ids);
    *ids.iter().min().unwrap()
}

fn vec_to_intervals(v: Vec<i64>) -> Vec<Interval> {
    v.chunks(2).map(|c| Interval(c[0], c[0] + c[1] - 1)).collect()
}

pub fn solve_part2(almanac: &Almanac) -> i64 {
    let ints = vec_to_intervals(almanac.seeds.clone());
    let ints = almanac.seed_to_soil.map_intervals(&ints);
    let ints = almanac.soil_to_fertilizer.map_intervals(&ints);
    let ints = almanac.fertilizer_to_water.map_intervals(&ints);
    let ints = almanac.water_to_light.map_intervals(&ints);
    let ints = almanac.light_to_temperature.map_intervals(&ints);
    let ints = almanac.temperature_to_humidity.map_intervals(&ints);
    let ints = almanac.humidity_to_location.map_intervals(&ints);
    ints.iter().map(|interval| interval.0).min().unwrap()
}

pub fn solve(input: &str) -> (i64, i64) {
    let (_,data) = parser::parse(input).unwrap();
    (solve_part1(&data), solve_part2(&data))
}

#[test]
fn test_day5_example() {
    let solution = solve(&include_str!("../inputs/day5-example"));
    assert_eq!(solution, (35, 46));
}

#[test]
fn test_day5_input() {
    let solution = solve(&include_str!("../inputs/day5-input"));
    assert_eq!(solution, (486613012, 56931769));
}
