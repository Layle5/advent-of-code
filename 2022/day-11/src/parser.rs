use nom::branch as nbr;
use nom::bytes::complete as nb;
use nom::character::complete as nc;
use nom::{multi as nm};
use nom::sequence as ns;
use nom::IResult;
use nom::combinator as nco;
use crate::{Monkey, Operand, Operation, Operator};

pub fn parse_monkeys_and_worry_levels(input: &str) -> Vec<(Monkey, Vec<u64>)> {
    let monkeys_result = nm::separated_list1(nm::many1(nc::line_ending), parse_monkey_and_worry_levels)(input);
    monkeys_result.expect("to parse monkey correctly").1
}

fn parse_monkey_and_worry_levels(input: &str) -> IResult<&str, (Monkey, Vec<u64>)> {
    let (input, (id, items, operation, divisibility_number, true_monkey_id, _, false_monkey_id)) = ns::tuple(
        (
            parse_monkey_id,
            parse_monkey_items,
            parse_monkey_operation,
            parse_monkey_divisibility_number,
            parse_monkey_next_id,
            nc::line_ending,
            parse_monkey_next_id,
        )
    )(input)?;

    Ok((input, (Monkey {
        id,
        operation,
        divisibility_number,
        true_monkey_id,
        false_monkey_id,
    }, items)))
}

fn parse_monkey_id(input: &str) -> IResult<&str, usize> {
    let (input, (_, id, _, _)) =
        ns::tuple((
            nb::tag("Monkey "),
            nc::u64,
            nc::char(':'),
            nc::line_ending,
        ))(input)?;

    Ok((input, id as usize))
}

fn parse_monkey_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, (_, items, _)) = ns::tuple((
        nb::tag("  Starting items: "),
        nm::separated_list1(
            ns::tuple((nc::char(','), nc::space1)),
            nc::u64,
        ),
        nc::line_ending,
    ))(input)?;

    Ok((input, items))
}

fn parse_monkey_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (_, operator, _, operand, _)) =
        ns::tuple((
            nb::tag("  Operation: new = old "),
            nbr::alt(
                (
                    nco::map(nb::tag("+"), |_| Operator::Add),
                    nco::map(nb::tag("*"), |_| Operator::Multiply),
                )
            ),
            nc::space1,
            nbr::alt(
                (
                    nco::map(nb::tag("old"), |_| Operand::Old),
                    nco::map(nc::u64, |n| Operand::Number(n)),
                )
            ),
            nc::line_ending,
        ))(input)?;

    Ok((input, Operation { operator, operand }))
}

fn parse_monkey_divisibility_number(input: &str) -> IResult<&str, u64> {
    let (input, (_, test_divisible, _)) =
        ns::tuple((
            nb::tag("  Test: divisible by "),
            nc::u64,
            nc::line_ending,
        ))(input)?;

    Ok((input, test_divisible))
}

fn parse_monkey_next_id(input: &str) -> IResult<&str, usize> {
    let (input, (_, _, _, true_monkey_id)) =
        ns::tuple((
            nb::tag("    If "),
            nm::many1(nc::alpha1),
            nb::tag(": throw to monkey "),
            nc::u64,
        ))(input)?;

    Ok((input, true_monkey_id as usize))
}
