use sha2::{Digest, Sha256};

pub fn str_checksum(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    let mut hex = String::with_capacity(digest.len() * 2);

    for byte in digest {
        use std::fmt::Write;
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }

    hex
}

#[cfg(test)]
mod tests {}
