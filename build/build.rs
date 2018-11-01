extern crate itertools;

use std::collections::HashMap;
use std::env;
use std::error;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU8;
use std::path::Path;
use std::result;

use itertools::Itertools;

type Result<T> = result::Result<T, Box<error::Error>>;

fn generate_inner(writer: &mut impl Write, mut atoms: Vec<(&[u8], NonZeroU8)>) -> Result<()> {
    let mut prefix = Vec::new();
    loop {
        let prefix_byte = {
            let (s, _) = atoms.first().unwrap();
            s[0]
        };

        if atoms.iter().all(|(s, _)| s[0] == prefix_byte) {
            prefix.push(prefix_byte);
            for (s, _) in atoms.iter_mut() {
                *s = &s[1..];
            }
        } else {
            break;
        }
    }

    if atoms.is_empty() {
        assert_eq!(b"", &prefix[..]);
    }

    if !prefix.is_empty() {
        writeln!(
            writer,
            "ArrayExpect::expect(&mut s, [{prefix}])?;",
            prefix = itertools::join(prefix, ", ")
        )?;
    }

    writeln!(writer, "match read(&mut s)? {{");

    for (prefix_byte, mut atoms) in &atoms.into_iter().group_by(|(s, _)| s[0]) {
        let atoms = atoms.map(|(s, num)| (&s[1..], num)).collect_vec();
        writeln!(writer, "{prefix_byte} => {{", prefix_byte = prefix_byte)?;
        if atoms.len() == 1 {
            let (suffix, num) = atoms.first().unwrap();
            if !suffix.is_empty() {
                writeln!(
                    writer,
                    "ArrayExpect::expect(&mut s, [{suffix}])?;",
                    suffix = itertools::join(*suffix, ", ")
                )?;
            }

            writeln!(
                writer,
                "Some(unsafe {{ NonZeroU8::new_unchecked({num}) }})",
                num = num.get()
            )?;
        } else {
            generate_inner(writer, atoms)?;
        }

        writeln!(writer, "}}")?;
    }

    writeln!(
        writer,
        "\
        _ => None,
    }}"
    );
    Ok(())
}

fn generate(mut writer: impl Write, mod_name: &str, atoms: Vec<&[u8]>) -> Result<()> {
    let mut by_len = HashMap::new();
    for (num, s) in atoms.into_iter().enumerate() {
        by_len
            .entry(s.len())
            .or_insert_with(Vec::new)
            .push((s, NonZeroU8::new(num as u8 + 1).unwrap()));
    }

    writeln!(
        writer,
        "\
    pub mod {mod_name} {{
        use super::*;

        pub fn from_str(s: &str) -> Option<NonZeroU8> {{
            let mut s = s.as_bytes();
            match s.len() {{",
        mod_name = mod_name
    )?;

    for (len, mut atoms) in by_len {
        writeln!(writer, "{len} => {{", len = len)?;
        atoms.sort();
        generate_inner(&mut writer, atoms)?;
        writeln!(writer, "}}")?;
    }

    writeln!(
        writer,
        "\
                _ => None,
            }}
        }}
    }}"
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let filename = Path::new(&env::var("OUT_DIR")?).join("atoms.rs");
    let mut file = File::create(filename)?;

    generate(&mut file, "small", vec![b"BTC-EUR", b"ETH-EUR", b"ETH-BTC"])?;

    generate(
        &mut file,
        "big",
        vec![
            b"BTC-EUR",
            b"ETH-EUR",
            b"ETH-BTC",
            b"BTC-USDC",
            b"ETH-USDC",
            b"ETC-BTC",
            b"ETC-EUR",
            b"BTC-USD",
            b"BCH-BTC",
            b"BCH-USD",
            b"BTC-GBP",
            b"ETH-USD",
            b"LTC-BTC",
            b"LTC-EUR",
            b"LTC-USD",
            b"BCH-EUR",
            b"ETC-USD",
            b"ZRX-USD",
            b"ZRX-BTC",
            b"ZRX-EUR",
            b"ETC-GBP",
            b"ETH-GBP",
            b"LTC-GBP",
            b"BCH-GBP",
        ],
    )?;

    Ok(())
}
