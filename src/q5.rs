pub fn solve_a(input: &str) -> String {
    let mut stacks = ElfStacks::new(&[
        "NRJTZBDF", "HJNSR", "QFZGJNRC", "QTRGNVF", "FQTL", "NGRBZWCQ", "MHNSLCF", "JTMQND", "SGP",
    ]);

    for line in input.split('\n') {
        let operation = Operation::from(line);

        let mut removed: Vec<char> = (0..operation.amount)
            .map(|_| stacks.pop_from_stack(operation.start - 1).unwrap())
            .collect();

        stacks.append_in_order(operation.end - 1, &mut removed);
    }

    stacks.tops().iter().collect()
}

pub fn solve_b(input: &str) -> String {
    let mut stacks = ElfStacks::new(&[
        "NRJTZBDF", "HJNSR", "QFZGJNRC", "QTRGNVF", "FQTL", "NGRBZWCQ", "MHNSLCF", "JTMQND", "SGP",
    ]);

    for line in input.split('\n') {
        let operation = Operation::from(line);

        let mut removed: Vec<char> = (0..operation.amount)
            .map(|_| stacks.pop_from_stack(operation.start - 1).unwrap())
            .collect();

        stacks.append_reverse(operation.end - 1, &mut removed);
    }

    stacks.tops().iter().collect()
}

struct ElfStacks(Vec<Vec<char>>);

impl ElfStacks {
    pub fn new(stacks: &[&str]) -> Self {
        Self(stacks.iter().map(|s| s.chars().rev().collect()).collect())
    }

    pub fn tops(&self) -> Vec<char> {
        self.0.iter().map(|s| *s.last().unwrap()).collect()
    }

    pub fn pop_from_stack(&mut self, i: usize) -> Option<char> {
        self.0.get_mut(i)?.pop()
    }

    pub fn append_in_order(&mut self, i: usize, vals: &[char]) {
        self.0.get_mut(i).unwrap().append(&mut vals.to_vec());
    }

    pub fn append_reverse(&mut self, i: usize, vals: &[char]) {
        let mut vals = vals.to_vec();
        vals.reverse();

        self.0.get_mut(i).unwrap().append(&mut vals.to_vec());
    }
}

struct Operation {
    pub amount: usize,
    pub start: usize,
    pub end: usize,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let amount = s[s.find("e ").unwrap() + 2..s.find(" f").unwrap()]
            .parse::<usize>()
            .unwrap();
        let start = s[s.find("m ").unwrap() + 2..s.find(" t").unwrap()]
            .parse::<usize>()
            .unwrap();
        let end = s[s.find("o ").unwrap() + 2..].parse::<usize>().unwrap();

        Self { amount, start, end }
    }
}
