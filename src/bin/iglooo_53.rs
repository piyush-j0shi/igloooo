// For the breaks_here function and the reason is because closure returns the
// smae type and if and else can not return diffrent types so we need to use the dyn and for the
// size we need to use Box because we do not know that on compile time that's why we are using
// Box<dyn Fn(i32) -> i32> as return type

//With impl Fn, the compiler knows exactly which closure it is, so the call goes straight to that code
//and can be inlined. With Box<dyn Fn>, it can't know until runtime which of the two closures it's holding,
//so the call goes through a pointer lookup to find the right code.
//That's dynamic dispatch, and it's the second thing we traded away.

// simple function can return single closure but breaks_here can return 2 clopsure and this makes a
// diffrence which is we need to use this method for the future use case we will trade some speed
// so I am going to use this until I find something else, Peace.

fn simple_function(n: i32) -> impl Fn(i32) -> i32 {
    move |x| n * x
}

fn breaks_here(flag: bool, n: i32) -> Box<dyn Fn(i32) -> i32> {
    if flag {
        Box::new(move |x| x * n)
    } else {
        Box::new(move |x: i32| -x * n)
    }
}

fn main() {
    let simple_variable = simple_function(5);
    println!("simple result : {}", simple_variable(3));

    let breakshere = breaks_here(false, 1);
    println!("breaks here : {}", breakshere(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let simple_variable = simple_function(5);
        assert_eq!(15, simple_variable(3));
    }

    #[test]
    fn test_breakshere_flag_false() {
        let breakshereflagfalse = breaks_here(false, 1);
        assert_eq!(-5, breakshereflagfalse(5));
    }

    #[test]
    fn test_breakshere_flag_true() {
        let breakshereflagtrue = breaks_here(true, 1);
        assert_eq!(5, breakshereflagtrue(5));
    }
}
