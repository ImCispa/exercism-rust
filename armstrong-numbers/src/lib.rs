pub fn is_armstrong_number(num: u32) -> bool {
    let power: usize = num.to_string().len();
    let mut total : u32 = 0; 
    for c in num.to_string().chars() {
        let n = c.to_digit(10).unwrap();
        total += n.pow(power.try_into().unwrap());
    }
    return total == num;

}
