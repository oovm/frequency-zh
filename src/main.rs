#![feature(try_trait)]

use crate::baike2018::count_baike2018;
use crate::wiki2019::count_wiki2019;

mod baike2018;
mod data;
mod wiki2019;

fn main() -> std::io::Result<()> {
    count_baike2018()?;
    count_wiki2019()?;
    Ok(())
}
