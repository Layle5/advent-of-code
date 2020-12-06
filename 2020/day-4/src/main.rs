#![feature(str_split_once)]

use std::collections::HashMap;
use std::env;
use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

fn parse_passport<'a>(paragraph: &[&'a str]) -> Passport<'a> {
    let fields: HashMap<_, _> = paragraph
        .iter()
        .flat_map(|paragraph_line: &&str| paragraph_line.split(" "))
        .map(|value_string: &str| value_string.split_once(':').unwrap())
        .collect();

    Passport { fields }
}

fn parse_passports<'a>(content: &'a str) -> Vec<Passport<'a>> {
    let lines: Vec<&str> = content.lines().collect();
    let paragraphs = lines.split(|s: &&str| s.len() == 0);
    let passports: Vec<Passport> = paragraphs
        .map(|paragraph: &[&str]| parse_passport(paragraph))
        .collect();

    passports
}

fn is_year(year_str: &str, min: u16, max: u16) -> bool {
    match year_str.parse::<u16>() {
        Err(_) => false,
        Ok(year) => year_str.len() == 4 && min <= year && year <= max,
    }
}

fn is_height(height_str: &str) -> bool {
    let units: &[(&str, u16, u16)] = &[("cm", 150, 193), ("in", 59, 76)];

    let option_unit = units
        .iter()
        .filter(|(unit_suffix, _, _)| height_str.ends_with(unit_suffix))
        .next();
    match option_unit {
        None => false,
        Some(&(_, min, max)) => {
            let number_len = height_str.len() - 2;
            let option_height = height_str[..number_len].parse::<u16>();
            match option_height {
                Err(_) => false,
                Ok(height) => min <= height && height <= max,
            }
        }
    }
}

fn is_regex(value: &str, regex: &str) -> bool {
    let re = Regex::new(regex).unwrap();
    re.is_match(value)
}

fn is_eye_color(value: &str) -> bool {
    let valid_colors: &[&str] =
        &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    valid_colors.contains(&value)
}

fn is_passport_valid(passport: &Passport) -> bool {
    let required_fields: &[(&str, fn(&str) -> bool)] = &[
        ("byr", |s: &str| is_year(s, 1920, 2002)),
        ("iyr", |s: &str| is_year(s, 2010, 2020)),
        ("eyr", |s: &str| is_year(s, 2020, 2030)),
        ("hgt", |s: &str| is_height(s)),
        ("hcl", |s: &str| is_regex(s, "^#[0-9a-f]{6}$")),
        ("ecl", |s: &str| is_eye_color(s)),
        ("pid", |s: &str| is_regex(s, "^[0-9]{9}$")),
    ];
    required_fields
        .iter()
        .all(|(field_key, validate_field_value)| {
            match passport.fields.get_key_value(field_key) {
                None => false,
                Some((_, field_value)) => validate_field_value(field_value),
            }
        })
}

fn solve(content: &str) {
    let passports = parse_passports(content);
    let valid_passport_count = passports
        .iter()
        .filter(|passport| is_passport_valid(passport))
        .count();
    println!("{}", valid_passport_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_filename: &str = "./res/input.txt";
    let filename: &str =
        args.get(1).map(|s| s.as_ref()).unwrap_or(default_filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    solve(&content);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_password_valid_test() {
        let test = |s: &str, e: bool| {
            let passport = parse_passport(&[s]);
            assert_eq!(is_passport_valid(&passport), e);
        };
        test("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926", false);
        test("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946", false);
        test("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277", false);
        test("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007", false);
        test("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f", true);
        test("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm", true);
        test("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022", true);
        test("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719", true);
    }

    #[test]
    fn is_regex_test() {
        assert_eq!(is_regex("#fffff", "^#[0-9a-f]{6}$"), false);
        assert_eq!(is_regex("#ffffff", "^#[0-9a-f]{6}$"), true);
        assert_eq!(is_regex("#fffffff", "^#[0-9a-f]{6}$"), false);
    }
}
