struct FirstStruct<I, F> {
    iter: I,
    f: F,
}

struct SecondStruct<I, P> {
    iter: I,
    f: P,
}

struct ThirdStruct<I> {
    iter: I,
    element: usize,
}

struct FourthStruct<I, J> {
    iter1: I,
    iter2: J,
}

struct FifthStruct<I, L> {
    iter: I,
    f: L,
    flag: bool,
}

impl<I, F, B> Iterator for FirstStruct<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        let intermediate = self.iter.next();
        match intermediate {
            Some(x) => Some((self.f)(x)),
            None => None,
        }
    }
}

impl<I, P> Iterator for SecondStruct<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let somevalue = self.iter.next();
            let x = somevalue?;
            if (self.f)(&x) {
                return Some(x);
            }
        }
    }
}

impl<I> Iterator for ThirdStruct<I>
where
    I: Iterator,
{
    type Item = (usize, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let somevalue = self.iter.next()?;
        let return_value = (self.element, somevalue);

        self.element += 1;
        Some(return_value)
    }
}

impl<I, J> Iterator for FourthStruct<I, J>
where
    I: Iterator,
    J: Iterator,
{
    type Item = (I::Item, J::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let iterator_1 = self.iter1.next()?;
        let iterator_2 = self.iter2.next()?;

        let iterator_result = (iterator_1, iterator_2);
        Some(iterator_result)
    }
}

impl<I, L> Iterator for FifthStruct<I, L>
where
    I: Iterator,
    L: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.flag {
            let somevalue = self.iter.next()?;

            if (self.f)(&somevalue) {
                Some(somevalue)
            } else {
                self.flag = false;
                None
            }
        } else {
            None
        }
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 3, 2];
    let numbers_1 = [String::from("1"), String::from("2"), String::from("3")];

    let somestruct = FirstStruct {
        iter: numbers.iter(),
        f: |x| x * 2,
    };

    for x in somestruct {
        println!("{}", x);
    }

    let somestruct_1 = SecondStruct {
        iter: numbers.iter(),
        f: |x: &&i32| *x + 1 > 2,
    };

    println!();
    for i in somestruct_1 {
        println!("{}", i);
    }

    let somestruct_2 = ThirdStruct {
        iter: numbers_1.iter(),
        element: 0,
    };

    println!();
    for i in somestruct_2 {
        println!("{:?}", i);
    }

    let somestruct_3 = FourthStruct {
        iter1: numbers_1.iter(),
        iter2: numbers.iter(),
    };

    println!();
    for i in somestruct_3 {
        println!("{:?}", i);
    }

    let somestruct_4 = FifthStruct {
        iter: numbers.iter(),
        f: |x: &&i32| **x < 4,
        flag: true,
    };

    println!();
    for x in somestruct_4 {
        println!("{}", x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn map_matches_std() {
        let v = [1, 2, 3, 4, 3, 2];
        let mine: Vec<i32> = FirstStruct {
            iter: v.iter(),
            f: |x: &i32| x * 2,
        }
        .collect();
        let theirs: Vec<i32> = v.iter().map(|x| x * 2).collect();
        assert_eq!(mine, theirs);
    }

    #[test]
    fn map_changes_the_item_type() {
        let v = [1, 2, 3];
        let mine: Vec<String> = FirstStruct {
            iter: v.iter(),
            f: |x: &i32| x.to_string(),
        }
        .collect();
        assert_eq!(
            mine,
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn map_on_empty_yields_nothing() {
        let v: [i32; 0] = [];
        let mine: Vec<i32> = FirstStruct {
            iter: v.iter(),
            f: |x: &i32| x * 2,
        }
        .collect();
        assert!(mine.is_empty());
    }

    #[test]
    fn filter_matches_std() {
        let v: Vec<i32> = (1..=20).collect();
        let mine: Vec<&i32> = SecondStruct {
            iter: v.iter(),
            f: |x: &&i32| **x % 7 == 0,
        }
        .collect();
        let theirs: Vec<&i32> = v.iter().filter(|x| **x % 7 == 0).collect();
        assert_eq!(mine, theirs);
        assert_eq!(mine, vec![&7, &14]);
    }

    #[test]
    fn filter_with_no_matches_is_empty() {
        let v = [1, 2, 3];
        let mine: Vec<&i32> = SecondStruct {
            iter: v.iter(),
            f: |x: &&i32| **x > 999,
        }
        .collect();
        assert!(mine.is_empty());
    }

    #[test]
    fn filter_works_on_non_copy_items() {
        let words = vec![
            String::from("error"),
            String::from("ok"),
            String::from("error"),
        ];
        let mine: Vec<&String> = SecondStruct {
            iter: words.iter(),
            f: |s: &&String| s.as_str() == "error",
        }
        .collect();
        assert_eq!(mine, vec!["error", "error"]);
    }

    #[test]
    fn filter_pulls_upstream_many_times_for_one_item() {
        let v: Vec<i32> = (1..=20).collect();
        let pulls = Cell::new(0);
        let counted = v.iter().inspect(|_| pulls.set(pulls.get() + 1));

        let mut f = SecondStruct {
            iter: counted,
            f: |x: &&i32| **x % 7 == 0,
        };
        assert_eq!(f.next(), Some(&7));
        assert_eq!(pulls.get(), 7);
    }

    #[test]
    fn enumerate_matches_std() {
        let v = ["a", "b", "c", "d"];
        let mine: Vec<(usize, &&str)> = ThirdStruct {
            iter: v.iter(),
            element: 0,
        }
        .collect();
        let theirs: Vec<(usize, &&str)> = v.iter().enumerate().collect();
        assert_eq!(mine, theirs);
    }

    #[test]
    fn enumerate_starts_at_zero() {
        let v = ["a", "b"];
        let mut it = ThirdStruct {
            iter: v.iter(),
            element: 0,
        };
        assert_eq!(it.next(), Some((0, &"a")));
        assert_eq!(it.next(), Some((1, &"b")));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn enumerate_on_empty_yields_nothing() {
        let v: [i32; 0] = [];
        let mine: Vec<(usize, &i32)> = ThirdStruct {
            iter: v.iter(),
            element: 0,
        }
        .collect();
        assert!(mine.is_empty());
    }

    #[test]
    fn zip_matches_std() {
        let a = [1, 2, 3, 4, 5];
        let b = ["x", "y", "z"];
        let mine: Vec<(&i32, &&str)> = FourthStruct {
            iter1: a.iter(),
            iter2: b.iter(),
        }
        .collect();
        let theirs: Vec<(&i32, &&str)> = a.iter().zip(b.iter()).collect();
        assert_eq!(mine, theirs);
    }

    #[test]
    fn zip_stops_at_the_shorter_side() {
        let a = [1, 2, 3, 4, 5];
        let b = ["x", "y", "z"];
        let n = FourthStruct {
            iter1: a.iter(),
            iter2: b.iter(),
        }
        .count();
        assert_eq!(n, b.len().min(a.len()));
        assert_eq!(n, 3);
    }

    #[test]
    fn zip_loses_one_item_from_the_longer_first_side() {
        let a = [1, 2, 3, 4, 5];
        let b = ["x", "y", "z"];

        let mut ia = a.iter();
        let mut ib = b.iter();
        {
            let z = FourthStruct {
                iter1: &mut ia,
                iter2: &mut ib,
            };
            assert_eq!(z.count(), 3);
        }
        assert_eq!(ia.collect::<Vec<_>>(), vec![&5]);
        assert!(ib.next().is_none());
    }

    #[test]
    fn zip_with_shorter_side_first_loses_nothing() {
        let a = [1, 2, 3, 4, 5];
        let b = ["x", "y", "z"];

        let mut ia = a.iter();
        let mut ib = b.iter();
        {
            let z = FourthStruct {
                iter1: &mut ib,
                iter2: &mut ia,
            };
            assert_eq!(z.count(), 3);
        }
        assert_eq!(ia.collect::<Vec<_>>(), vec![&4, &5]);
    }

    #[test]
    fn take_while_matches_std() {
        let v = [1, 2, 3, 10, 4, 5];
        let mine: Vec<&i32> = FifthStruct {
            iter: v.iter(),
            f: |x: &&i32| **x < 5,
            flag: true,
        }
        .collect();
        let theirs: Vec<&i32> = v.iter().take_while(|x| **x < 5).collect();
        assert_eq!(mine, theirs);
        assert_eq!(mine, vec![&1, &2, &3]);
    }

    #[test]
    fn take_while_does_not_resume_after_stopping() {
        let v = [1, 2, 3, 10, 4, 5];
        let mut it = FifthStruct {
            iter: v.iter(),
            f: |x: &&i32| **x < 5,
            flag: true,
        };
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn take_while_stops_without_draining_upstream() {
        let mine: Vec<u32> = FifthStruct {
            iter: 1u32..,
            f: |x: &u32| *x < 6,
            flag: true,
        }
        .collect();
        assert_eq!(mine, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn take_while_failing_on_the_first_item_yields_nothing() {
        let v = [1, 2, 3];
        let mine: Vec<&i32> = FifthStruct {
            iter: v.iter(),
            f: |x: &&i32| **x > 99,
            flag: true,
        }
        .collect();
        assert!(mine.is_empty());
    }

    #[test]
    fn take_while_passing_everything_yields_everything() {
        let v = [1, 2, 3];
        let mine: Vec<&i32> = FifthStruct {
            iter: v.iter(),
            f: |_: &&i32| true,
            flag: true,
        }
        .collect();
        assert_eq!(mine, vec![&1, &2, &3]);
    }
}
