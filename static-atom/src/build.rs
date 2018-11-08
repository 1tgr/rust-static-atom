use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io::Write;
use std::result;
use std::str;

use itertools::Itertools;

type Result<T> = result::Result<T, Box<error::Error>>;

struct ByteStrDisplay<'a>(&'a [u8]);

impl<'a> fmt::Display for ByteStrDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(s) = str::from_utf8(self.0) {
            write!(f, "b{:?}", s)
        } else {
            write!(f, "&[{}]", itertools::join(self.0, ", "))
        }
    }
}

fn generate_inner<W: Write>(writer: &mut W, lower_name: &str, atoms: Vec<(&[u8], &str)>) -> Result<()> {
    for (_prefix_byte, mut atoms) in &atoms.into_iter().group_by(|&(s, _)| s[0]) {
        let mut atoms = atoms.collect_vec();
        let mut prefix = Vec::new();

        let s = loop {
            let &(bytes, s) = atoms.first().unwrap();

            let prefix_byte = if let Some(&b) = bytes.first() {
                b
            } else {
                break Some(s);
            };

            if atoms.iter().all(|(s, _)| s[0] == prefix_byte) {
                prefix.push(prefix_byte);
                for (s, _) in atoms.iter_mut() {
                    *s = &s[1..];
                }
            } else {
                break None;
            }
        };

        writeln!(
            writer,
            "if let Some(s) = s.expect({prefix}) {{",
            prefix = ByteStrDisplay(&prefix[..])
        )?;

        if let Some(s) = s {
            write!(writer, "Ok({lower_name}!({s:?}))", lower_name = lower_name, s = s)?;
        } else {
            generate_inner(writer, lower_name, atoms)?;
        }

        write!(writer, "}} else ")?;
    }

    writeln!(writer, "{{ Err(()) }}")?;
    Ok(())
}

pub fn generate<W: Write>(mut writer: W, name: &str, atoms: Vec<&str>) -> Result<()> {
    let lower_name = name.to_lowercase();

    let mut by_len = HashMap::new();
    for &s in atoms.iter() {
        let bytes = s.as_bytes();
        by_len.entry(bytes.len()).or_insert_with(Vec::new).push((bytes, s));
    }

    writeln!(
        writer,
        "\
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum {name} {{",
        name = name
    );

    for (index, &s) in atoms.iter().enumerate() {
        writeln!(writer, "_{index}, // {s:?}", index = index, s = s);
    }

    writeln!(
        writer,
        "\
        }}

        #[macro_export]
        macro_rules! {lower_name} {{",
        lower_name = lower_name
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        writeln!(
            writer,
            "({s:?}) => {{ $crate::{name}::_{index} }};",
            name = name,
            index = index,
            s = s
        )?;
    }

    writeln!(
        writer,
        "\
         }}

         impl ::std::str::FromStr for {name} {{
            type Err = ();

            #[allow(unused_variables)]
            fn from_str(s: &str) -> ::std::result::Result<Self, ()> {{
                use ::static_atom::Expect;

                let s = s.as_bytes();
                match s.len() {{",
        name = name
    )?;

    for (len, mut atoms) in by_len.into_iter().sorted_by_key(|&(len, _)| len) {
        writeln!(writer, "{len} => {{", len = len)?;
        atoms.sort_by_key(|&(bytes, _)| bytes);
        generate_inner(&mut writer, &lower_name, atoms)?;
        writeln!(writer, "}}")?;
    }

    writeln!(
        writer,
        "\
                _ => Err(())
                }}
            }}
        }}

        impl {name} {{
            pub fn as_str(&self) -> &'static str {{
                match self {{",
        name = name
    )?;

    for &s in atoms.iter() {
        writeln!(writer, "{lower_name}!({s:?}) => {s:?},", lower_name = lower_name, s = s)?;
    }

    writeln!(
        writer,
        "\
                }}
            }}
        }}

        impl From<{name}> for usize {{
            fn from(token: {name}) -> Self {{
                match token {{",
        name = name
    )?;

    for (index, s) in atoms.iter().enumerate() {
        writeln!(
            writer,
            "{lower_name}!({s:?}) => {index},",
            lower_name = lower_name,
            index = index,
            s = s
        )?;
    }

    writeln!(
        writer,
        "\
                }}
            }}
        }}

        impl ::std::fmt::Debug for {name} {{
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{
                write!(f, \"{lower_name}!({{}})\", self.as_str())
            }}
        }}

        impl ::std::fmt::Display for {name} {{
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{
                f.write_str(self.as_str())
            }}
        }}",
        lower_name = lower_name,
        name = name
    )?;

    Ok(())
}
