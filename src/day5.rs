use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl FromStr for Almanac {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("\n\n");
        let seeds = if let Some(line) = iter.next() {
            if let Some((lhs, rhs)) = line.split_once(':') {
                if lhs != "seeds" {
                    return Err(s.to_string());
                } else {
                    let mut seeds = Vec::new();
                    let rhs = rhs.trim();
                    for num in rhs.split_whitespace() {
                        seeds.push(num.parse::<usize>().map_err(|e| e.to_string())?);
                    }
                    seeds
                }
            } else {
                return Err(s.to_string());
            }
        } else {
            return Err(s.to_string());
        };
        let mut maps = Vec::with_capacity(7);
        for block in iter {
            maps.push(block.parse::<Map>()?);
        }
        if maps.len() != 7 {
            Err(s.to_string())
        } else {
            macro_rules! err_if_not {
                ($x:ident, $src:ident, $dst:ident) => {
                    if !$x.has_src_dst(&Garden::$src, &Garden::$dst) {
                        return Err(s.to_string());
                    }
                };
            }
            let humidity_to_location = maps.pop().unwrap();
            err_if_not!(humidity_to_location, Humidity, Location);
            let temperature_to_humidity = maps.pop().unwrap();
            err_if_not!(temperature_to_humidity, Temperature, Humidity);
            let light_to_temperature = maps.pop().unwrap();
            err_if_not!(light_to_temperature, Light, Temperature);
            let water_to_light = maps.pop().unwrap();
            err_if_not!(water_to_light, Water, Light);
            let fertilizer_to_water = maps.pop().unwrap();
            err_if_not!(fertilizer_to_water, Fertilizer, Water);
            let soil_to_fertilizer = maps.pop().unwrap();
            err_if_not!(soil_to_fertilizer, Soil, Fertilizer);
            let seed_to_soil = maps.pop().unwrap();
            err_if_not!(seed_to_soil, Seed, Soil);
            Ok(Almanac {
                seeds,
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            })
        }
    }
}

impl Almanac {
    pub fn location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.lookup(seed);
        let fertilizer = self.soil_to_fertilizer.lookup(soil);
        let water = self.fertilizer_to_water.lookup(fertilizer);
        let light = self.water_to_light.lookup(water);
        let temperature = self.light_to_temperature.lookup(light);
        let humidity = self.temperature_to_humidity.lookup(temperature);
        self.humidity_to_location.lookup(humidity)
    }

    pub fn locations_part1(&self) -> impl Iterator<Item = usize> + '_ {
        self.seeds.iter().map(|&seed| self.location(seed))
    }

    pub fn minimum_location<'a, F, T>(&'a self, f: F) -> usize
    where
        F: Fn(&'a Almanac) -> T,
        T: Iterator<Item = usize> + 'a,
    {
        f(&self).fold(usize::MAX, |acc, x| acc.min(x))
    }

    pub fn minimum_location_part1(&self) -> usize {
        self.minimum_location(|x| x.locations_part1())
    }

    pub fn locations_part2(&self) -> impl Iterator<Item = usize> + '_ {
        assert_eq!(self.seeds.len() & 1, 0);
        self.seeds.chunks_exact(2).flat_map(|w| {
            let start = w[0];
            let len = w[1];
            (start..start + len).map(|seed| self.location(seed))
        })
    }
    pub fn minimum_location_part2(&self) -> usize {
        self.minimum_location(|x| x.locations_part2())
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let s = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
        s.parse::<Self>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Garden {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

// macro_rules! from_err {
//     {$T:path, $U:path, $V:ident} => {
//         impl From<$T> for $U {
//             fn from(e: $T) -> Self {
//                 Self::$V(e)
//             }
//         }
//     }
// }

impl FromStr for Garden {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Garden::*;
        match s {
            "seed" => Ok(Seed),
            "soil" => Ok(Soil),
            "fertilizer" => Ok(Fertilizer),
            "water" => Ok(Water),
            "light" => Ok(Light),
            "temperature" => Ok(Temperature),
            "humidity" => Ok(Humidity),
            "location" => Ok(Location),
            _ => Err(s.to_string()),
        }
    }
}

impl Garden {
    pub fn dst(&self) -> Option<Self> {
        use Garden::*;
        match self {
            Seed => Some(Soil),
            Soil => Some(Fertilizer),
            Fertilizer => Some(Water),
            Water => Some(Light),
            Light => Some(Temperature),
            Temperature => Some(Humidity),
            Humidity => Some(Location),
            Location => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    ranges: Vec<SrcDst>,
    src: Garden,
    dst: Garden,
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((lhs, rhs)) = s.split_once(':') {
            let lhs = lhs.trim_end_matches(" map");
            if let Some((src, dst)) = lhs.split_once("-to-") {
                let src = src.parse::<Garden>()?;
                let dst = dst.parse::<Garden>()?;
                let mut ranges = Vec::new();
                let rhs = rhs.trim();
                for line in rhs.lines() {
                    ranges.push(line.parse::<SrcDst>()?);
                }
                Ok(Map::new(ranges, src, dst))
            } else {
                Err(s.to_string())
            }
        } else {
            Err(s.to_string())
        }
    }
}

impl Map {
    pub fn new(mut ranges: Vec<SrcDst>, src: Garden, dst: Garden) -> Self {
        // Sorting is a necessary condition to use binary search in `lookup`
        ranges.sort_unstable_by(|a, b| a.src.cmp(&b.src));
        Self { ranges, src, dst }
    }

    /*
    With O(n) lookup:

    time ./target/release/day5 ./input/day5.txt
    579439039
    7873084

    real	1m51.358s
    user	1m51.169s
    sys	0m0.001s

    ----

    With O(lgn) lookup:

    time ./target/release/day5 ./input/day5.txt
    579439039
    7873084

    real	1m11.060s
    user	1m10.729s
    sys	0m0.005s
    */
    pub fn lookup(&self, i: usize) -> usize {
        match self.ranges.binary_search_by(|x| x.src.cmp(&i)) {
            Ok(mid) => match self.ranges[mid].lookup(i) {
                Some(j) => j,
                None => i,
            },
            Err(0) => i,
            Err(left) => match self.ranges[left - 1].lookup(i) {
                Some(j) => j,
                None => i,
            },
        }
    }
    // pub fn lookup(&self, i: usize) -> usize {
    //     // With a bit more effort, this could be converted to a binary search,
    //     // hence, O(lgn) rather than O(n).
    //     for srcdst in self.ranges.iter() {
    //         match srcdst.lookup(i) {
    //             Some(x) => return x,
    //             None => (),
    //         }
    //     }
    //     i
    // }

    pub fn has_src_dst(&self, src: &Garden, dst: &Garden) -> bool {
        self.src == *src && self.dst == *dst
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrcDst {
    src: usize,
    dst: usize,
    len: usize,
}

impl FromStr for SrcDst {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match (iter.next(), iter.next(), iter.next()) {
            (Some(dst), Some(src), Some(len)) => {
                let len = len.parse::<usize>().map_err(|e| e.to_string())?;
                let dst = dst.parse::<usize>().map_err(|e| e.to_string())?;
                let src = src.parse::<usize>().map_err(|e| e.to_string())?;
                Ok(Self { src, dst, len })
            }
            _ => Err(s.to_string()),
        }
    }
}

impl SrcDst {
    pub fn new(src: usize, dst: usize, len: usize) -> Self {
        Self { src, dst, len }
    }
    pub fn lookup(&self, i: usize) -> Option<usize> {
        let j = i.wrapping_sub(self.src);
        if j >= self.len {
            None
        } else {
            Some(self.dst + j)
        }
    }
}

/*
We can take a different approach and move some of the run-time checks into typestate.

This is much work little difference here, but, in general, can be quite useful
when encoding complex sequences of transformations.
*/

pub struct Map2<T: Relation> {
    ranges: Vec<SrcDst>,
    _marker: T,
}

pub trait Relation {}

pub struct SeedSoil;
pub struct SoilFertilizer;
pub struct FertilizerWater;
pub struct WaterLight;
pub struct LightTemperature;
pub struct TemperatureHumidity;
pub struct HumidityLocation;

pub struct Seed(usize);
pub struct Soil(usize);
pub struct Fertilizer(usize);
pub struct Water(usize);
pub struct Light(usize);
pub struct Temperature(usize);
pub struct Humidity(usize);
pub struct Location(usize);

macro_rules! impl_relation {
    { $($T:ident)+ } => {
        $( impl Relation for $T {} )+
    }
}
impl_relation! { SeedSoil SoilFertilizer FertilizerWater WaterLight LightTemperature TemperatureHumidity HumidityLocation }

macro_rules! impl_from_str {
    { $($T:ident)+ ; $($S:expr)+ } => {
        $(
            impl FromStr for $T {
                type Err = String;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    if s.trim() == $S {
                        Ok(Self)
                    } else {
                        Err(s.to_string())
                    }
                }
            }
        )+
    }
}
impl_from_str! { SeedSoil SoilFertilizer FertilizerWater WaterLight LightTemperature TemperatureHumidity HumidityLocation ;
                 "seed-to-soil" "soil-to-fertilizer" "fertilizer-to-water" "water-to-light" "light-to-temperature" "temperature-to-humidity" "humidity-to-location"
}

impl<T: Relation + FromStr<Err = String>> FromStr for Map2<T> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((lhs, rhs)) = s.split_once(':') {
            let lhs = lhs.trim_end_matches(" map");
            let marker = lhs.parse::<T>()?;
            let mut ranges = Vec::new();
            let rhs = rhs.trim();
            for line in rhs.lines() {
                ranges.push(line.parse::<SrcDst>()?);
            }
            Ok(Map2::new(ranges, marker))
        } else {
            Err(s.to_string())
        }
    }
}

impl<T: Relation> Map2<T> {
    pub fn new(ranges: Vec<SrcDst>, _marker: T) -> Self {
        Self { ranges, _marker }
    }
    pub fn lookup_imp(&self, i: usize) -> usize {
        match self.ranges.binary_search_by(|x| x.src.cmp(&i)) {
            Ok(mid) => match self.ranges[mid].lookup(i) {
                Some(j) => j,
                None => i,
            },
            Err(0) => i,
            Err(left) => match self.ranges[left - 1].lookup(i) {
                Some(j) => j,
                None => i,
            },
        }
    }
}

macro_rules! impl_lookup {
    {$T:ident, $U:ident, $V:ident} => {
        impl Map2<$T> {
            pub fn lookup(&self, i: $U) -> $V {
                $V(self.lookup_imp(i.0))
            }
        }
    }
}
impl_lookup! { SeedSoil, Seed, Soil }
impl_lookup! { SoilFertilizer, Soil, Fertilizer }
impl_lookup! { FertilizerWater, Fertilizer, Water }
impl_lookup! { WaterLight, Water, Light}
impl_lookup! { LightTemperature, Light, Temperature}
impl_lookup! { TemperatureHumidity, Temperature, Humidity}
impl_lookup! { HumidityLocation, Humidity, Location }

pub struct Almanac2 {
    seeds: Vec<usize>,
    seed_to_soil: Map2<SeedSoil>,
    soil_to_fertilizer: Map2<SoilFertilizer>,
    fertilizer_to_water: Map2<FertilizerWater>,
    water_to_light: Map2<WaterLight>,
    light_to_temperature: Map2<LightTemperature>,
    temperature_to_humidity: Map2<TemperatureHumidity>,
    humidity_to_location: Map2<HumidityLocation>,
}

impl FromStr for Almanac2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("\n\n");
        let seeds = if let Some(line) = iter.next() {
            if let Some((lhs, rhs)) = line.split_once(':') {
                if lhs != "seeds" {
                    return Err(s.to_string());
                } else {
                    let mut seeds = Vec::new();
                    let rhs = rhs.trim();
                    for num in rhs.split_whitespace() {
                        seeds.push(num.parse::<usize>().map_err(|e| e.to_string())?);
                    }
                    seeds
                }
            } else {
                return Err(s.to_string());
            }
        } else {
            return Err(s.to_string());
        };
        macro_rules! parse_or_err {
            ($T:ident) => {
                match iter.next().map(|s| s.parse::<Map2<$T>>()) {
                    Some(Ok(map)) => map,
                    _ => return Err(s.to_string()),
                }
            };
        }
        let seed_to_soil = parse_or_err!(SeedSoil);
        let soil_to_fertilizer = parse_or_err!(SoilFertilizer);
        let fertilizer_to_water = parse_or_err!(FertilizerWater);
        let water_to_light = parse_or_err!(WaterLight);
        let light_to_temperature = parse_or_err!(LightTemperature);
        let temperature_to_humidity = parse_or_err!(TemperatureHumidity);
        let humidity_to_location = parse_or_err!(HumidityLocation);
        Ok(Almanac2 {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl Almanac2 {
    pub fn location(&self, seed: Seed) -> usize {
        // This sequence is now guaranteed through the use of the typestate pattern.
        // If someone were to mistakenly change the order, it would not longer compile.
        let soil = self.seed_to_soil.lookup(seed);
        let fertilizer = self.soil_to_fertilizer.lookup(soil);
        let water = self.fertilizer_to_water.lookup(fertilizer);
        let light = self.water_to_light.lookup(water);
        let temperature = self.light_to_temperature.lookup(light);
        let humidity = self.temperature_to_humidity.lookup(temperature);
        self.humidity_to_location.lookup(humidity).0
    }

    pub fn locations_part1(&self) -> impl Iterator<Item = usize> + '_ {
        self.seeds.iter().map(|&seed| self.location(Seed(seed)))
    }

    pub fn minimum_location<'a, F, T>(&'a self, f: F) -> usize
    where
        F: Fn(&'a Almanac2) -> T,
        T: Iterator<Item = usize> + 'a,
    {
        f(&self).fold(usize::MAX, |acc, x| acc.min(x))
    }

    pub fn minimum_location_part1(&self) -> usize {
        self.minimum_location(|x| x.locations_part1())
    }

    pub fn locations_part2(&self) -> impl Iterator<Item = usize> + '_ {
        assert_eq!(self.seeds.len() & 1, 0);
        self.seeds.chunks_exact(2).flat_map(|w| {
            let start = w[0];
            let len = w[1];
            (start..start + len).map(|seed| self.location(Seed(seed)))
        })
    }
    pub fn minimum_location_part2(&self) -> usize {
        self.minimum_location(|x| x.locations_part2())
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let s = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
        s.parse::<Self>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn srcdst_lookup() {
        let x = SrcDst {
            src: 98,
            dst: 50,
            len: 2,
        };

        assert_eq!(x.lookup(98), Some(50));
        assert_eq!(x.lookup(99), Some(51));
        assert_eq!(x.lookup(100), None);

        let x = SrcDst {
            src: 25,
            dst: 18,
            len: 70,
        };
        assert_eq!(x.lookup(81), Some(74));
    }

    #[test]
    fn map_lookup() {
        let map = Map::new(
            vec![SrcDst::new(98, 50, 2), SrcDst::new(50, 52, 48)],
            Garden::Seed,
            Garden::Soil,
        );
        assert_eq!(map.lookup(0), 0);
        assert_eq!(map.lookup(1), 1);
        assert_eq!(map.lookup(48), 48);
        assert_eq!(map.lookup(49), 49);
        assert_eq!(map.lookup(50), 52);
        assert_eq!(map.lookup(51), 53);
        assert_eq!(map.lookup(79), 81);
        assert_eq!(map.lookup(96), 98);
        assert_eq!(map.lookup(97), 99);
        assert_eq!(map.lookup(98), 50);
        assert_eq!(map.lookup(99), 51);
        assert_eq!(map.lookup(100), 100);

        let map = Map::new(
            vec![
                SrcDst {
                    src: 18,
                    dst: 88,
                    len: 7,
                },
                SrcDst {
                    src: 25,
                    dst: 18,
                    len: 70,
                },
            ],
            Garden::Water,
            Garden::Light,
        );
        assert_eq!(map.lookup(81), 74);
    }

    #[test]
    fn map_from_str() {
        let s = "\
seed-to-soil map:
50 98 2
52 50 48";
        assert_eq!(
            s.parse::<Map>().unwrap(),
            Map::new(
                vec![SrcDst::new(98, 50, 2), SrcDst::new(50, 52, 48)],
                Garden::Seed,
                Garden::Soil,
            )
        );
    }

    #[test]
    fn almanac_from_str() {
        let x = TEST.parse::<Almanac>().unwrap();
        assert_eq!(x.seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn locations_part1() {
        let x = TEST.parse::<Almanac>().unwrap();
        let lhs: Vec<_> = x.locations_part1().collect();
        assert_eq!(lhs, vec![82, 43, 86, 35]);
    }

    #[test]
    fn minimum_location_part1() {
        let x = TEST.parse::<Almanac>().unwrap();
        assert_eq!(x.minimum_location_part1(), 35);
    }

    #[test]
    fn minimum_location_part2() {
        let x = TEST.parse::<Almanac>().unwrap();
        assert_eq!(x.minimum_location_part2(), 46);
    }
}
