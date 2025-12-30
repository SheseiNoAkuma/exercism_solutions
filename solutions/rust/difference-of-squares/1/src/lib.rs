pub fn square_of_sum(n: u32) -> u32 {
    numbers(n).iter().sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    numbers(n).iter().map(|d| d.pow(2)).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn numbers(n: u32) -> Vec<u32> {
    (1..=n).collect::<Vec<u32>>()
}
