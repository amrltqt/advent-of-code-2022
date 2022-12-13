use std::str;
#[derive(Debug, Clone)]
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

fn parse_row(line: &str, base_size: usize) -> Vec<Option<Crate>> {
    let mut row: Vec<Option<Crate>> = vec![None; base_size];
    line.to_string()
        .as_bytes()
        .chunks(4)
        .map(str::from_utf8)
        .filter_map(|r| r.ok())
        .enumerate()
        .for_each(|(index, token)| {
            if token.starts_with("[") {
                row[index] = Some(Crate::from(token.trim()));
            }
        });
    row
}

fn transpose(rows: &mut Vec<Vec<Option<Crate>>>, nb_stacks: usize) -> Vec<Stack> {
    
    // reverse row to have the last item in the beginning of stacks
    rows.reverse();

    let mut stacks: Vec<Stack> = Vec::new();
    for i in 0..nb_stacks {
        let mut stack = Stack::new();
        rows.iter().for_each(|row| {
            if let Some(a_crate) = row[i].as_ref() {
                stack.push(a_crate.clone());
            }
        });
        stacks.push(stack);
    }

    stacks
}

fn parse_crates(raw_starter: &str) -> Vec<Stack> {
    let rows = raw_starter.split("\n").collect::<Vec<_>>();
    let (raw_crates, raw_indexes) = rows.split_at(rows.len() - 1);

    let base_size = raw_indexes.first().unwrap().len() / 4 + 1;

    let mut crates = raw_crates.to_vec()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(move |line| parse_row(line, base_size))
        .collect::<Vec<_>>();


    transpose(&mut crates, base_size)
}


fn main() {
    let input = advent_of_code_2022::load_day("5");
    let (raw_starter, raw_moves) = input.split_once("\n\n").unwrap();

    let moves = raw_moves
        .split("\n")
                .filter(|line| !line.is_empty())

        .map(Move::from)
        .collect::<Vec<_>>();

    let stacks = parse_crates(raw_moves);


}




#[cfg(test)]
mod test {
    use crate::{Move, parse_row, parse_crates};

    #[test]
    fn test_parse_move() {
        let test_case = "move 2 from 2 to 9";
        let rmove = Move::from(test_case);
        
        assert_eq!(rmove.quantity, 2);
    }

    #[test]
    fn test_parse_row() {
        let raw = "        [C] [B] [H] ";
        let row = parse_row(raw, 9);

        assert_eq!(row.len(), 9);
        assert_eq!(row[2].as_ref().unwrap().item, 'C');
    }

    #[test]
    fn test_parse_stack() {
        let input = r#"        [C] [B] [H]                
[W]     [D] [J] [Q] [B]            
[P] [F] [Z] [F] [B] [L]            
[G] [Z] [N] [P] [J] [S] [V]        
[Z] [C] [H] [Z] [G] [T] [Z]     [C]
[V] [B] [M] [M] [C] [Q] [C] [G] [H]
[S] [V] [L] [D] [F] [F] [G] [L] [F]
[B] [J] [V] [L] [V] [G] [L] [N] [J]
 1   2   3   4   5   6   7   8   9 "#;
        let stacks = parse_crates(input);

        assert_eq!(stacks[0].len(), 7);
        assert_eq!(stacks[1].len(), 6);

        assert_eq!(stacks[8].len(), 4);
    }

}