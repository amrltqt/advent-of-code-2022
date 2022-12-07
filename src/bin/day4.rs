struct Section {
    start: u32,
    end: u32
}

impl From<&str> for Section {
    fn from(raw: &str) -> Self {
        let (start, end) = raw.split_once('-').unwrap();
        Section {
            start: start.parse().unwrap(),
            end: end.parse().unwrap()
        }
    }
}


impl Section {
    fn is_contained_in(&self, section: &Section) -> bool {
        self.start >= section.start && self.end <= section.end
    }

    fn overlap(&self, section: &Section) -> bool {
        (
            section.start <= self.start && self.start <= section.end
        ) 
        ||
        (
            section.start <= self.end && self.end <= section.end 
        )
    }
}


fn main() {
    let input = advent_of_code_2022::load_day("4");
    let v: i32 = input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let first_section = Section::from(first);
            let second_section = Section::from(second);

            if first_section.is_contained_in(&second_section) || second_section.is_contained_in(&first_section) {
                return 1
            }
            return 0
        })
        .sum();
    println!("{}", v);

    let v: i32 = input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let first_section = Section::from(first);
            let second_section = Section::from(second);

            if first_section.overlap(&second_section) || second_section.overlap(&first_section) {
                return 1
            }
            return 0
        })
        .sum();
    println!("{}", v);
}


#[cfg(test)]
mod tests {
    use crate::Section;

    #[test]
    fn test_parsing() {
        let input = "2-4";
        let section = Section::from(input);

        assert_eq!(section.start, 2);
        assert_eq!(section.end, 4);
    }

    #[test]
    fn test_contained_in() {
        let a = Section {
            start: 2,
            end: 4
        };

        let b = Section {
            start: 2,
            end: 3
        };

        assert_eq!(a.is_contained_in(&b), false);
        assert_eq!(b.is_contained_in(&a), true);   
    }

    #[test]
    fn test_overlap() {
        let a = Section {
            start: 1,
            end: 2
        };

        let b = Section {
            start: 3,
            end: 4
        };

        assert_eq!(a.overlap(&b), false);

        let a = Section {
            start: 1,
            end: 2
        };

        let b = Section {
            start: 2,
            end: 4
        };

        assert_eq!(a.overlap(&b), true);

        let a = Section {
            start: 1,
            end: 3
        };

        let b = Section {
            start: 3,
            end: 4
        };

        assert_eq!(a.overlap(&b), true);

        let a = Section {
            start: 3,
            end: 3
        };

        let b = Section {
            start: 3,
            end: 4
        };

        assert_eq!(a.overlap(&b), true);

    }
}