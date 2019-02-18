use strs;
use strs::AppError;

fn main() -> Result<(), AppError> {
    strs::run()?;
    Ok(())
}
