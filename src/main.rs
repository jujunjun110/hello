use std::num::ParseIntError;
// type Result<T> = std::result::Result<T, ParseIntError>;

fn half_number(s: &str) -> Result<i32, ParseIntError> {
    let res = s.parse::<i32>();
    match res {
        Ok(n) => Ok(n / 2),
        Err(e) => Err(e),
    }
}

fn half_number2(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| n / 2)
}

fn half_number3(s: &str) -> Result<i32, ParseIntError> {
    let res = s.parse::<i32>()?;
    return Ok(res / 2);
}

fn half_number4(s: &str) -> Result<i32, &str> {
    let res = s.parse::<i32>();
    match res {
        Ok(n) => Ok(n / 2),
        Err(_) => Err("パースに失敗しました"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = "100x";
    let res = half_number3(n)?;
    println!("{}", res);

    Ok(())
}
