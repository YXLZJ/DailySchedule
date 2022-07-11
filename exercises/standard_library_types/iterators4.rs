// iterators4.rs



pub fn factorial(num: u64) -> u64 {
    fn help(a:u64,b:u64)->u64{
        if b==1 {
            return a;
        }else{
            return help(a*b,b-1);
        }
    }
    help(1,num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
