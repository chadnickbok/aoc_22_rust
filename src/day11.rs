use crate::utils;
use std::str::FromStr;

const MONKEY_MODULO: i64 = 19 * 7 * 17 * 13 * 11 * 2 * 5 * 3;

struct Monkey {
    inspectionCount: usize,
    op: fn(i64) -> i64,
    testFn: fn(i64) -> bool,
    trueMonkey: usize,
    falseMonkey: usize,
    curItems: Vec<i64>,
}

impl Monkey {
    pub fn inspect(&mut self) -> Vec<(usize, i64)> {
        let mut throws = Vec::new();
        self.inspectionCount += self.curItems.len();

        for item in self.curItems.iter() {
            let mut newItem = (self.op)(*item);
            newItem = newItem;

            if (self.testFn)(newItem) {
                throws.push((self.trueMonkey, newItem));
            } else {
                throws.push((self.falseMonkey, newItem));
            }
        }

        self.curItems.clear();

        throws
    }
}

pub fn star1(filename: &str) -> Result<i64, utils::AocError> {
    let mut monkeys = Vec::new();

    // Monkey 0
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| x * 3 % MONKEY_MODULO,
        testFn: |x| x % 13 == 0,
        trueMonkey: 6,
        falseMonkey: 2,
        curItems: vec![89, 73, 66, 57, 64, 80],
    });

    // Monkey 1
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| x + 1,
        testFn: |x| x % 3 == 0,
        trueMonkey: 7,
        falseMonkey: 4,
        curItems: vec![83, 78, 81, 55, 81, 59, 69],
    });

    // Monkey 2
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| x * 13 % MONKEY_MODULO,
        testFn: |x| x % 7 == 0,
        trueMonkey: 1,
        falseMonkey: 4,
        curItems: vec![76, 91, 58, 85],
    });

    // Monkey 3
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| {
            (x * x) % MONKEY_MODULO
        },
        testFn: |x| x % 2 == 0,
        trueMonkey: 6,
        falseMonkey: 0,
        curItems: vec![71, 72, 74, 76, 68],
    });

    // Monkey 4
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| x + 7,
        testFn: |x| x % 19 == 0,
        trueMonkey: 5,
        falseMonkey: 7,
        curItems: vec![98, 85, 84],
    });

    // Monkey 5
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| x + 8,
        testFn: |x| x % 5 == 0,
        trueMonkey: 3,
        falseMonkey: 0,
        curItems: vec![78],
    });

    // Monkey 6
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| x + 4,
        testFn: |x| x % 11 == 0,
        trueMonkey: 1,
        falseMonkey: 2,
        curItems: vec![86, 70, 60, 88, 88, 78, 74, 83],
    });

    // Monkey 7
    monkeys.push(Monkey{
        inspectionCount: 0,
        op: |x| x + 5,
        testFn: |x| x % 17 == 0,
        trueMonkey: 3,
        falseMonkey: 5,
        curItems: vec![81, 58],
    });

    for i in 0..10000 {
        println!("ROUND {}", i);
        for m in 0..monkeys.len() {
            let mut cur_monkey = &mut monkeys[m];
            let throws = cur_monkey.inspect();
            
            for throw in throws {
                monkeys[throw.0].curItems.push(throw.1);
            }
        }
    }

    for i in 0..monkeys.len() {
        println!("{} {}", i, monkeys[i].inspectionCount);
    }

    Ok(0)
}