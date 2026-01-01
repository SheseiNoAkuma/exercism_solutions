#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut total_value: u64 = 0;
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        total_value = total_value * from_base as u64 + digit as u64;
    }

    if total_value == 0 {
        return Ok(vec![0]);
    }

    let mut result = Vec::new();
    while total_value > 0 {
        result.push((total_value % to_base as u64) as u32);
        total_value /= to_base as u64;
    }
    result.reverse();

    Ok(result)
}
