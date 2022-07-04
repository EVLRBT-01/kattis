use std::str::FromStr;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");

    let result = triangle_area(&input.trim());

    println!("{}", result);
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
