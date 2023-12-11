use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let almanac = parse_almanac(input);

    let normal_seeds = almanac
        .seeds
        .iter()
        .map(|seed| (*seed..*seed + 1))
        .collect_vec();

    let minimum_location_for_normal_seeds = find_minimum_mapped_location(&almanac, normal_seeds);
    println!("Part 1: {minimum_location_for_normal_seeds}");

    let ranges_seeds = almanac
        .seeds
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];
            start..start + length
        })
        .collect_vec();

    let minimum_location_for_ranges_seeds = find_minimum_mapped_location(&almanac, ranges_seeds);
    println!("Part 2: {minimum_location_for_ranges_seeds}");
}

fn find_minimum_mapped_location(almanac: &Almanac, initial_ranges_seeds: Vec<Range>) -> Seed {
    let ranges_locations =
        almanac
            .maps
            .iter()
            .fold(initial_ranges_seeds, |mut ranges_seeds, map| {
                let mut new_ranges_seeds = vec![];

                while let Some(range_seed) = ranges_seeds.pop() {
                    let find_option = map.mappings.iter().find_map(|mapping| {
                        mapping
                            .overlap(&range_seed)
                            .map(|overlap| (mapping, overlap))
                    });

                    if let Some((mapping, overlap)) = find_option {
                        let mapped_start = mapping.map(overlap.start);
                        let mapped_end = mapping.map(overlap.end);
                        new_ranges_seeds.push(mapped_start..mapped_end);

                        let left_start = range_seed.start;
                        let left_end = overlap.start;
                        if left_start < left_end {
                            ranges_seeds.push(left_start..left_end);
                        }

                        let right_start = overlap.end;
                        let right_end = range_seed.end;
                        if right_start < right_end {
                            ranges_seeds.push(right_start..right_end);
                        }
                    } else {
                        new_ranges_seeds.push(range_seed);
                    }
                }

                new_ranges_seeds
            });

    ranges_locations
        .into_iter()
        .map(|range| range.start)
        .min()
        .expect("minimum range location")
}

fn parse_almanac(input: &str) -> Almanac {
    let (input, (_, seeds, _)) = nom::sequence::tuple::<_, _, nom::error::Error<_>, _>((
        nom::bytes::complete::tag("seeds: "),
        nom::multi::separated_list1(
            nom::character::complete::space1::<&str, nom::error::Error<_>>,
            nom::character::complete::u64,
        ),
        nom::multi::many0(nom::character::complete::line_ending),
    ))(input)
    .expect("seeds for almanac");

    let maps = nom::multi::separated_list1::<_, _, _, nom::error::Error<_>, _, _>(
        nom::multi::many1(nom::character::complete::line_ending),
        nom::sequence::tuple((
            nom::bytes::complete::is_not(" "),
            nom::bytes::complete::tag(" map:"),
            nom::character::complete::line_ending,
            nom::multi::separated_list1(
                nom::character::complete::line_ending,
                nom::multi::separated_list1(
                    nom::character::complete::space1,
                    nom::character::complete::u64,
                ),
            ),
        )),
    )(input)
    .expect("maps for almanac")
    .1
    .into_iter()
    .map(|map_tuple| {
        let (name, _, _, mappings_tuples) = map_tuple;
        let mut mappings = mappings_tuples
            .into_iter()
            .map(|mapping_tuple| Mapping {
                length: mapping_tuple[2],
                source_start: mapping_tuple[1],
                destination_start: mapping_tuple[0],
            })
            .collect_vec();

        mappings.sort_by(|lhs, rhs| lhs.source_start.cmp(&rhs.source_start));
        Map { name, mappings }
    })
    .collect_vec();

    Almanac { seeds, maps }
}

#[derive(Debug, Clone)]
struct Almanac<'a> {
    seeds: Vec<Seed>,
    maps: Vec<Map<'a>>,
}

#[derive(Debug, Clone)]
struct Map<'a> {
    #[allow(dead_code)]
    name: &'a str,
    mappings: Vec<Mapping>,
}

#[derive(Debug, Clone)]
struct Mapping {
    length: Seed,
    source_start: Seed,
    destination_start: Seed,
}

impl Mapping {
    fn source_end(&self) -> Seed {
        self.source_start + self.length
    }

    fn overlap(&self, range: &Range) -> Option<Range> {
        let overlap_start = range.start.max(self.source_start);
        let overlap_end = range.end.min(self.source_end());
        if overlap_end <= overlap_start {
            None
        } else {
            Some(overlap_start..overlap_end)
        }
    }

    fn map(&self, seed: Seed) -> Seed {
        seed + self.destination_start - self.source_start
    }
}

type Range = std::ops::Range<Seed>;

type Seed = u64;
