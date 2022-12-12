use std::str;
#[derive(Debug)]
struct Crate {
    item: char
}

impl From<&str> for Crate {
    fn from(raw: &str) -> Self {
        if raw.len() == 3 && raw.starts_with("[") && raw.ends_with("]") {
            return Crate {
                item: raw.chars().nth(1).unwrap()
            };
        }
        panic!("wtf");
    }
}
type Stack = Vec<Crate>;
type Cargo = Vec<Stack>;

#[derive(Debug)]
struct Move {
    quantity: u32,
    from: u32,
    to: u32,
}

fn tokenize(raw: &str) -> Vec<String> {
    raw.split(" ")
        .map(|raw| raw.to_string())
        .collect::<Vec<_>>()
}

impl From<&str> for Move {
    fn from(raw: &str) -> Self {
        let tokens = tokenize(raw);
        let mut itokens = tokens.iter();
        itokens.next();
        let quantity = itokens.next().unwrap().parse::<u32>().unwrap();
        itokens.next();
        let from = itokens.next().unwrap().parse::<u32>().unwrap();
        itokens.next();
        let to = itokens.next().unwrap().parse::<u32>().unwrap();

        Self {
            quantity,
            from,
            to
        }
    }
}

fn parse_row(line: &str) -> Vec<Option<Crate>> {
   line.to_string()
        .as_bytes()
        .chunks(4)
        .map(str::from_utf8)
        .filter_map(|r| r.ok())
        .map(|token| {
            if token.starts_with("[") {
                Some(Crate::from(token.trim()))
            } else {
                None
            }
        })
        .collect::<Vec<Option<Crate>>>()
}

fn main() {
    let input = advent_of_code_2022::load_day("5");
    let (raw_starter, raw_moves) = input.split_once("\n\n").unwrap();

    let moves = raw_moves
        .split("\n")
                .filter(|line| !line.is_empty())

        .map(Move::from)
        .collect::<Vec<_>>();

    let rows = raw_starter
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(parse_row)
        .collect::<Vec<_>>();

        println!("{:?}", moves);
        println!("{:?}", rows);

}


#[cfg(test)]
mod test {
    use crate::{Move, parse_row};

    #[test]
    fn test_parse_move() {
        let test_case = "move 2 from 2 to 9";
        let rmove = Move::from(test_case);
        
        assert_eq!(rmove.quantity, 2);
    }

    #[test]
    fn test_parse_row() {
        let raw = "        [C] [B] [H] ";
        let row = parse_row(raw);

        println!("{:?}", row);
    }

}