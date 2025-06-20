use anyhow::Result;
use rswin32::ntexapi2::internal::*;

fn main() -> Result<()> {
    let res = query_system_information::<SYSTEM_PERFORMANCE_INFORMATION>()?;
    println!("{}", res.first().unwrap().SystemCalls);
    Ok(())
}
