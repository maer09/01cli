use crate::{cli::Base64Format, get_reader};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),

        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // 移除读到的空白符（包括换行符）
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };
    
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::UrlSafe;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/output.b64";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
