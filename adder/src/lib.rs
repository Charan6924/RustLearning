pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(left : u64) -> u64{
    left + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two(){
        let result = add_two(2);
        assert_eq!(result,4, "Not equal to 4");
    }
}
