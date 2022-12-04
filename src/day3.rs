use crate::utils;


pub fn char_to_score(c: char) -> usize {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

pub fn star1(filename: &str) -> usize {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut total = 0;

    for raw_line in lines {
        match raw_line {
            Ok(line) => {
                let mid = line.len() / 2;
                let l = &line[..mid];
                let r  = &line[mid..];

                let mut found = false;
                for lc in l.chars() {
                    for rc in r.chars() {
                        if lc == rc {
                            let s = char_to_score(lc);
                            println!("{} {}", lc, s);
                            total += s;
                            found = true;
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }
            },
            _ => break,
        }
    }

    total
}


fn common_score(elf_group: &Vec<String>) -> Option<usize> {
    let a = elf_group.get(0).expect("failed to get line 0");
    let b = elf_group.get(1).expect("failed to get element 2");
    let c = elf_group.get(2).expect("failed to get element 3");

    for ac in a.chars() {
        for bc in b.chars() {
            for cc in c.chars() {
                if ac == bc && bc == cc {
                    let score = char_to_score(ac);
                    println!("found common {} {}", ac, score);
                    return Some(score);
                }
            }
        }
    }
    None
}

pub fn star2(filename: &str) -> Result<usize, std::io::Error> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let mut total = 0;
    let mut elf_group = Vec::new();

    for raw_line in lines {
        let line = raw_line.expect("failed parsing line");
        elf_group.push(line);

        if elf_group.len() == 3 {
            total += common_score(&elf_group).expect("failed to get common item");
            elf_group.clear();
        }
    }

    Ok(total)
}