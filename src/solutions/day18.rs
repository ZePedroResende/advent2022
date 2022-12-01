use crate::{load_file, AoCDay};
pub struct Code;

#[derive(Clone)]
enum Element {
    Val(i32),
    Pair(Box<Element>, Box<Element>),
}

impl Element {
    fn parse(s: &[u8], i: usize) -> (usize, Self) {
        match s[i] {
            b'[' => {
                let (i, e1) = Self::parse(s, i + 1);
                let (i, e2) = Self::parse(s, i + 1);
                (i + 1, Self::Pair(e1.boxed(), e2.boxed()))
            }
            _ => (i + 1, Self::Val((s[i] - b'0') as i32)),
        }
    }

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn depth(&self) -> usize {
        match self {
            Self::Val(_) => 0,
            Self::Pair(e1, e2) => 1 + std::cmp::max(e1.depth(), e2.depth()),
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Self::Val(x) => *x,
            Self::Pair(e1, e2) => 3 * e1.magnitude() + 2 * e2.magnitude(),
        }
    }

    fn split(&self) -> Option<Self> {
        match self {
            Self::Val(x) if *x >= 10 => {
                let a = *x / 2;
                let b = *x - a;
                return Some(Self::Pair(Self::Val(a).boxed(), Self::Val(b).boxed()));
            }
            Self::Pair(e1, e2) => {
                if let Some(a) = e1.split() {
                    return Some(Self::Pair(a.boxed(), e2.clone()));
                }
                if let Some(b) = e2.split() {
                    return Some(Self::Pair(e1.clone(), b.boxed()));
                }
            }
            _ => {}
        }
        None
    }

    fn add_left(&self, e2: Option<Self>) -> Self {
        match (self, e2) {
            (_, None) => self.clone(),
            (Self::Val(x), Some(Self::Val(x2))) => Self::Val(*x + x2),
            (Self::Pair(a, b), e2) => Self::Pair(a.add_left(e2).boxed(), b.clone()),
            _ => panic!(),
        }
    }

    fn add_right(&self, e2: Option<Self>) -> Self {
        match (self, e2) {
            (_, None) => self.clone(),
            (Self::Val(x), Some(Self::Val(x2))) => Self::Val(*x + x2),
            (Self::Pair(a, b), e2) => Self::Pair(a.clone(), b.add_right(e2).boxed()),
            _ => panic!(),
        }
    }

    fn explode(&self, n: usize) -> Option<(Option<Self>, Self, Option<Self>)> {
        if let Self::Pair(e1, e2) = self {
            if n == 1 {
                return Some((Some(*e1.clone()), Self::Val(0), Some(*e2.clone())));
            }
            if let Some((left, a, right)) = e1.explode(n - 1) {
                let e = Self::Pair(a.boxed(), e2.add_left(right).boxed());
                return Some((left, e, None));
            }
            if let Some((left, b, right)) = e2.explode(n - 1) {
                let e = Self::Pair(e1.add_right(left).boxed(), b.boxed());
                return Some((None, e, right));
            }
        }
        None
    }

    fn reduce(mut self) -> Self {
        loop {
            let d = self.depth();
            if d > 4 {
                if let Some((_, e2, _)) = self.explode(d) {
                    self = e2;
                    continue;
                }
            }
            match self.split() {
                Some(e2) => self = e2,
                None => break self,
            }
        }
    }
}

fn add(e1: &Element, e2: &Element) -> Element {
    Element::Pair(e1.clone().boxed(), e2.clone().boxed()).reduce()
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let elements = data
            .lines()
            .map(|l| Element::parse(l.as_bytes(), 0).1)
            .collect::<Vec<_>>();

        let out = elements[1..]
            .iter()
            .fold(elements[0].clone(), |e1, e2| add(&e1, e2))
            .magnitude();

        out.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let elements = data
            .lines()
            .map(|l| Element::parse(l.as_bytes(), 0).1)
            .collect::<Vec<_>>();

        let tuples = elements
            .iter()
            .enumerate()
            .flat_map(|(i, a)| elements[i + 1..].iter().map(move |b| (a, b)));

        let out = tuples
            .flat_map(|(e1, e2)| [add(e1, e2).magnitude(), add(e2, e1).magnitude()])
            .max()
            .unwrap();

        out.to_string()
    }
}
