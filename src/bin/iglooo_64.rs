struct Interleave<A, B> {
    a: A,
    b: B,
    a_turn: bool,
}

impl<A, B> Iterator for Interleave<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let a_turn = self.a_turn;
        self.a_turn = !a_turn;

        if a_turn {
            self.a.next().or_else(|| self.b.next())
        } else {
            self.b.next().or_else(|| self.a.next())
        }
    }
}

fn interleave<A, B>(a: A, b: B) -> Interleave<A::IntoIter, B::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator<Item = A::Item>,
{
    Interleave {
        a: a.into_iter(),
        b: b.into_iter(),
        a_turn: true,
    }
}

fn main() {
    let merged: Vec<&i32> = interleave([1, 2, 3].iter(), [1, 3].iter()).collect();
    println!("{:?}", merged);
}

#[cfg(test)]
mod tests {
    use super::interleave;

    #[test]
    fn equal_lengths_alternate() {
        let got: Vec<i32> = interleave(vec![1, 2, 3], vec![10, 20, 30]).collect();
        assert_eq!(got, vec![1, 10, 2, 20, 3, 30]);
    }

    #[test]
    fn first_side_shorter_drains_the_second() {
        let got: Vec<i32> = interleave(vec![1], vec![10, 20, 30]).collect();
        assert_eq!(got, vec![1, 10, 20, 30]);
    }

    #[test]
    fn second_side_shorter_drains_the_first() {
        let got: Vec<i32> = interleave(vec![1, 2, 3], vec![10]).collect();
        assert_eq!(got, vec![1, 10, 2, 3]);
    }

    #[test]
    fn first_side_empty_yields_all_of_the_second() {
        let got: Vec<i32> = interleave(Vec::new(), vec![10, 20]).collect();
        assert_eq!(got, vec![10, 20]);
    }

    #[test]
    fn second_side_empty_yields_all_of_the_first() {
        let got: Vec<i32> = interleave(vec![1, 2], Vec::new()).collect();
        assert_eq!(got, vec![1, 2]);
    }

    #[test]
    fn both_empty_yields_nothing() {
        let got: Vec<i32> = interleave(Vec::<i32>::new(), Vec::new()).collect();
        assert!(got.is_empty());
    }

    #[test]
    fn stays_none_past_exhaustion() {
        let mut it = interleave(vec![1], vec![10]);
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(10));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn works_on_non_copy_items() {
        let left = vec![String::from("alpha"), String::from("gamma")];
        let right = vec![String::from("beta")];
        let got: Vec<String> = interleave(left, right).collect();
        assert_eq!(got, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn accepts_two_different_iterator_types() {
        let owned = vec![String::from("one"), String::from("three")];
        let borrowed = "two\nfour";
        let got: Vec<String> = interleave(owned, borrowed.lines().map(String::from)).collect();
        assert_eq!(got, vec!["one", "two", "three", "four"]);
    }
}
