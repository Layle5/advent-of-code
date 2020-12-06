use std::env;
use std::fs;

fn summarize<'a, I>(questions_iter: I) -> [usize; 26]
where
    I: IntoIterator<Item = &'a u8>,
{
    questions_iter.into_iter().fold(
        [0; 26],
        |mut form_summary: [usize; 26], &question: &u8| {
            let question_index: usize = (question - b'a').into();
            form_summary[question_index] += 1;
            form_summary
        },
    )
}

fn count_common_answers<'a, I>(number_people: usize, form_summary: I) -> usize
where
    I: IntoIterator<Item = &'a usize>,
{
    form_summary
        .into_iter()
        .filter(|&&question_count: &&usize| question_count == number_people)
        .count()
}

fn solve(content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    let count = lines
        .split(|&line: &&str| line.is_empty())
        .map(|paragraph: &[&str]| {
            let number_people = paragraph.len();
            let questions_iter =
                paragraph.iter().flat_map(|line| line.as_bytes());
            (number_people, questions_iter)
        })
        .map(|(number_people, questions_iter): (usize, _)| {
            let form_summary = summarize(questions_iter);
            (number_people, form_summary)
        })
        .map(|(number_people, form_summary): (usize, [usize; 26])| {
            count_common_answers(number_people, &form_summary)
        })
        .fold(0, |sum, form_count| sum + form_count);

    println!("{}", count)
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
mod tests {}
