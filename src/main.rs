use std::str::FromStr;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");

    let result = final_exam(input.trim(), |_| {
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("could not read input");
        answer.trim().to_string()
    });

    println!("{}", result);
}

fn final_exam<T: Fn(usize) -> String>(questions: &str, answer: T) -> String {
    let questions =
        usize::from_str(questions).expect(&format!("could not parse {} as a number", questions));
    let mut previous = "".to_string();
    (0..questions)
        .map(|q| {
            let answer = answer(q);
            let correct = if answer == previous { 1 } else { 0 };
            previous = answer;
            correct
        })
        .sum::<u32>()
        .to_string()
}

fn count_the_vowels(input: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    input
        .to_lowercase()
        .chars()
        .filter(|c| VOWELS.contains(c))
        .count()
        .to_string()
}

fn jack_o_lantern(input: &str) -> String {
    input
        .split(" ")
        .map(|v| u32::from_str(v).expect(&format!("could not parse {} as a number", v)))
        .reduce(|total, value| total * value)
        .expect("expected a total")
        .to_string()
}

use std::collections::HashSet;

fn hangman<'a>(word: &str, guesses: &str) -> &'a str {
    const MAX_GUESSES: usize = 10;

    let mut word: HashSet<char> = word.chars().collect();
    let mut wrong_guesses = 0;
    for letter in guesses.chars() {
        if !word.remove(&letter) {
            wrong_guesses += 1;
        }
        if word.is_empty() || wrong_guesses == MAX_GUESSES {
            break;
        }
    }

    if word.is_empty() {
        "WIN" // Player 2 guessed the word
    } else {
        "LOSE" // Player 1 wins as player 2 could not guess the word
    }
}

fn triangle_area(input: &str) -> String {
    let values: Vec<f32> = input
        .split(" ")
        .map(|v| f32::from_str(v).expect(&format!("could not parse {} as a number", v)))
        .collect();

    let height = values.first().expect("expected a number for the height");
    let base = values.last().expect("expected a number for the base");
    format!("{}", 0.5 * base * height)
}

fn two_sum(input: &str) -> String {
    let values: Vec<u16> = input
        .split(" ")
        .map(|v| u16::from_str(v).expect(&format!("could not parse {} as a number", v)))
        .collect();

    format!("{}", values.iter().sum::<u16>())
}

fn greater(input: &str) -> String {
    let values: Vec<usize> = input
        .split(" ")
        .map(|v| usize::from_str(v).expect(&format!("could not parse {} as a number", v)))
        .collect();

    format!("{}", (values.first() > values.last()) as u8)
}

fn echo(input: &str) -> String {
    format!("{} {} {}", input, input, input)
}

fn ninety_nine_problems(input: &str) -> String {
    let value = usize::from_str(&input).expect("could not parse input as number");

    let remainder = value % 100;
    let upper_difference = 99 - remainder;
    let lower_difference = remainder + 1;

    let result = if lower_difference < upper_difference && value >= 100 {
        value - lower_difference
    } else {
        value + upper_difference
    };
    format!("{}", result)
}

#[cfg(test)]
mod tests {

    #[test]
    fn final_exam() {
        assert_eq!(crate::final_exam("4", |_| "A".to_string()), "3");
        assert_eq!(
            crate::final_exam("6", |q| {
                match q {
                    0 => "A".to_string(),
                    1 => "D".to_string(),
                    2 => "B".to_string(),
                    3 => "B".to_string(),
                    4 => "C".to_string(),
                    5 => "A".to_string(),
                    _ => panic!(),
                }
            }),
            "1"
        );
    }

    #[test]
    fn count_the_vowels() {
        assert_eq!(crate::count_the_vowels("This is a test."), "4");
        assert_eq!(crate::count_the_vowels("How many vowels in sky?"), "5");
        assert_eq!(
            crate::count_the_vowels("Can you handle both CAPITAL and lower case?"),
            "14"
        );
        assert_eq!(
            crate::count_the_vowels("D. J. Pike flung Q. V. Schwartz my box."),
            "5"
        );
    }

    #[test]
    fn jack_o_lantern() {
        assert_eq!(crate::jack_o_lantern("3 4 5"), "60");
        assert_eq!(crate::jack_o_lantern("2 2 2"), "8");
        assert_eq!(crate::jack_o_lantern("3 1 5"), "15");
    }

    #[test]
    fn hangman() {
        assert_eq!(
            crate::hangman("HANGMAN", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            "WIN"
        );
        assert_eq!(
            crate::hangman("BANANA", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            "LOSE"
        );
        assert_eq!(
            crate::hangman("RAINBOWS", "USIANBVLOJRKWXZCTQGHPFMYDE"),
            "WIN"
        );
    }

    #[test]
    fn triangle_area() {
        assert_eq!(crate::triangle_area("1 1"), "0.5");
        assert_eq!(crate::triangle_area("2 2"), "2");
        assert_eq!(crate::triangle_area("1000 1000"), "500000");
    }

    #[test]
    fn two_sum() {
        assert_eq!(crate::two_sum("0 0"), "0");
        assert_eq!(crate::two_sum("1 1"), "2");
        assert_eq!(crate::two_sum("2 2"), "4");
        assert_eq!(crate::two_sum("1000 1000"), "2000");
    }

    #[test]
    fn greater() {
        assert_eq!(crate::greater("1 19"), "0");
        assert_eq!(crate::greater("4 4"), "0");
        assert_eq!(crate::greater("23 14"), "1");
        assert_eq!(crate::greater("1000000000 999999999"), "1");
    }

    #[test]
    fn echo() {
        assert_eq!(crate::echo("Hello!"), "Hello! Hello! Hello!")
    }

    #[test]
    fn ninety_nine_problems() {
        for i in 1..=10_000 {
            println!("{i} -> {}", crate::ninety_nine_problems(&i.to_string()));

            let u = std::cmp::max(i / 100, 1) * 100;
            let expected = if i < u + 49 { u - 1 } else { u + 99 };
            assert_eq!(
                crate::ninety_nine_problems(&i.to_string()),
                expected.to_string()
            )
        }
    }
}
