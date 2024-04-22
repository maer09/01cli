use std::{fs::File, io::Read};
use anyhow::Result;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 用Box和syn还有Trait消除多种类型
    let reader: Box<dyn Read> = if input == "-" {
        // windows命令行下下按ctrl+z输入EOF才能终止输入
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}