use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let report = parse_report(input).unwrap().1;
    let extrapolations = report.histories.iter().map(|h| extrapolate(&h.values)).collect_vec();
    let summed_extrapolations = extrapolations.iter().map(|e| e.1).sum::<i64>();
    println!("Part 1: {}", summed_extrapolations);
    let summed_extrapolations = extrapolations.iter().map(|e| e.0).sum::<i64>();
    println!("Part 2: {}", summed_extrapolations);
}

fn extrapolate(values: &[i64]) -> (i64, i64) {
    let first = values.first().copied().unwrap_or_default();
    let last = values.last().copied().unwrap_or_default();
     if values.iter().all_equal() {
        (first, last)
    } else {
        let differences = values.iter().copied().tuple_windows().map(|(lhs, rhs)| rhs - lhs).collect_vec();
        let (difference_first, difference_last) = extrapolate(&differences);
        (first - difference_first, last + difference_last)
    }
}

type ParseError<'a> = nom::error::Error<&'a str>;
type ParseResult<'a, I> = nom::IResult<&'a str, I, ParseError<'a>>;

fn parse_report(input: &str) -> ParseResult<Report> {
    let (input, all_values) = nom::multi::separated_list1(
        nom::character::complete::line_ending,
        nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::i64,
        ),
    )(input)?;

    let histories = all_values.into_iter().map(|values| History { values }).collect_vec();

    Ok((input, Report { histories }))
}

#[derive(Debug, Clone)]
struct Report {
    histories: Vec<History>,
}

#[derive(Debug, Clone)]
struct History {
    values: Vec<i64>,
}
