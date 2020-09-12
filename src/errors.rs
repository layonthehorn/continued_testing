use anyhow::Result;

pub fn propagating_errors() -> Result<(i32)> {
    let number: i32 = "34i".trim().parse()?;

    Ok(number)
}
