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

pub fn generate<W: Write>(
    mut writer: W,
    mod_name: &str,
    name: &str,
    lower_name: &str,
    atoms: Vec<&str>,
    visitors: Vec<&str>,
) -> Result<()> {
    let mod_name = if mod_name.is_empty() {
        String::new()
    } else {
        mod_name.to_owned() + "::"
    };

    let mut by_len = HashMap::new();
    for &s in atoms.iter() {
        let bytes = s.as_bytes();
        by_len.entry(bytes.len()).or_insert_with(Vec::new).push((bytes, s));
    }

    writeln!(
        writer,
        "\
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

        #[doc(hide)]
        pub mod _{lower_name}_types {{",
        lower_name = lower_name
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        writeln!(
            writer,
            "\
            #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct _{index}; // {s:?}",
            index = index,
            s = s
        );
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
            "({s:?}) => {{ $crate::{mod_name}{name}::_{index} }};",
            mod_name = mod_name,
            name = name,
            index = index,
            s = s
        )?;
    }

    writeln!(
        writer,
        "\
        }}

        #[macro_export]
        macro_rules! {lower_name}_type {{",
        lower_name = lower_name
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        writeln!(
            writer,
            "({s:?}) => {{ $crate::{mod_name}_{lower_name}_types::_{index} }};",
            mod_name = mod_name,
            lower_name = lower_name,
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
            }}"
    )?;

    for visitor in visitors.iter() {
        writeln!(
            writer,
            "\
            pub fn visit_{lower_visitor}<V: {visitor}Visitor>(self, visitor: V) -> V::Value {{
                match self {{",
            visitor = visitor,
            lower_visitor = visitor.to_lowercase(),
        )?;

        for s in atoms.iter() {
            writeln!(
                writer,
                "{lower_name}!({s:?}) => visitor.visit::<{lower_name}_type!({s:?})>(),",
                lower_name = lower_name,
                s = s,
            )?;
        }

        writeln!(
            writer,
            "\
                }}
            }}"
        )?;
    }

    writeln!(
        writer,
        "\
        }}

        impl From<{name}> for usize {{
            fn from(token: {name}) -> Self {{
                match token {{",
        name = name
    )?;

    for (index, &s) in atoms.iter().enumerate() {
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

        impl ::static_atom::TryFrom<usize> for {name} {{
            type Err = ();

            fn try_from(n: usize) -> Result<Self, ()> {{
                match n {{",
        name = name
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        writeln!(
            writer,
            "{index} => Ok({lower_name}!({s:?})),",
            lower_name = lower_name,
            index = index,
            s = s
        )?;
    }

    writeln!(
        writer,
        "\
                    _ => Err(()),
                }}
            }}
        }}"
    );

    #[cfg(feature = "serde")]
    {
        writeln!(
            writer,
            "\
            impl ::serde::Serialize for {name} {{
                fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {{
                    self.as_str().serialize(serializer)
                }}
            }}

            impl<'de> ::serde::Deserialize<'de> for {name} {{
                fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
                    use serde::de::Error;

                    <&str>::deserialize(deserializer).and_then(|s| {{
                        s.parse().map_err(|()| Error::custom(format!(\"can't parse {{}} as {name}\", s)))
                    }})
                }}
            }}",
            name = name
        );
    }

    let where_mapping = atoms
        .iter()
        .map(|&s| {
            format!(
                "M: ::static_atom::Mapping<{lower_name}_type!({s:?})>",
                lower_name = lower_name,
                s = s
            )
        })
        .join("\n,");

    writeln!(
        writer,
        "\
        impl ::std::fmt::Debug for {name} {{
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{
                write!(f, \"{lower_name}!({{}})\", self.as_str())
            }}
        }}

        impl ::std::fmt::Display for {name} {{
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{
                f.write_str(self.as_str())
            }}
        }}

        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct {name}Map<T>([Option<T>; {len}]);
        
        impl<T> {name}Map<T> {{
            pub fn new() -> Self {{
                {name}Map(Default::default())
            }}
        }}
        
        impl<T> ::static_atom::AtomMap for {name}Map<T> {{
            type Key = {name};
            type Value = T;
        
            fn entry(&self, key: {name}) -> &Option<T> {{
                &self.0[usize::from(key)]
            }}
        
            fn entry_mut(&mut self, key: {name}) -> &mut Option<T> {{
                &mut self.0[usize::from(key)]
            }}
        
            fn entries(&self) -> &[Option<T>] {{
                &self.0
            }}
        
            fn entries_mut(&mut self) -> &mut [Option<T>] {{
                &mut self.0
            }}
        }}

        impl<T> ::std::iter::FromIterator<({name}, T)> for {name}Map<T> {{
            fn from_iter<I: ::std::iter::IntoIterator<Item = ({name}, T)>>(iter: I) -> Self {{
                let mut map = {name}Map::new();
                for (key, value) in iter {{
                    map.0[usize::from(key)] = Some(value);
                }}

                map
            }}
        }}

        pub struct Typed{name}Map<M>
        where {where_mapping}
        {{",
        lower_name = lower_name,
        name = name,
        len = atoms.len(),
        where_mapping = where_mapping
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        writeln!(
            writer,
            "_{index}: Option<<M as ::static_atom::Mapping<{lower_name}_type!({s:?})>>::Value>,",
            index = index,
            lower_name = lower_name,
            s = s
        )?;
    }

    writeln!(
        writer,
        "\
        }}

        impl<M> Typed{name}Map<M>
        where {where_mapping}
        {{
            pub fn new() -> Self {{
                Typed{name}Map {{",
        name = name,
        where_mapping = where_mapping
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        writeln!(writer, "_{index}: None, // {s:?}", index = index, s = s)?;
    }

    writeln!(
        writer,
        "\
                }}
            }}
        }}

        impl<M> ::static_atom::TypedAtomMap<M> for Typed{name}Map<M>
        where {where_mapping}
        {{
            fn entry<A>(&self) -> &Option<<M as ::static_atom::Mapping<A>>::Value>
            where
                M: ::static_atom::Mapping<A>,
                A: 'static,
            {{
                use std::any::TypeId;
                use std::mem;

                let id = TypeId::of::<A>();",
        name = name,
        where_mapping = where_mapping
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        write!(
            writer,
            "\
            if id == TypeId::of::<{lower_name}_type!({s:?})>() {{
                unsafe {{ mem::transmute(&self._{index}) }}
            }} else ",
            lower_name = lower_name,
            index = index,
            s = s
        )?;
    }

    writeln!(
        writer,
        "\
                {{
                    unreachable!()
                }}
            }}

            fn entry_mut<A>(&mut self) -> &mut Option<<M as ::static_atom::Mapping<A>>::Value>
            where
                M: ::static_atom::Mapping<A>,
                A: 'static,
            {{
                use std::any::TypeId;
                use std::mem;

                let id = TypeId::of::<A>();"
    )?;

    for (index, &s) in atoms.iter().enumerate() {
        write!(
            writer,
            "\
            if id == TypeId::of::<{lower_name}_type!({s:?})>() {{
                unsafe {{ mem::transmute(&mut self._{index}) }}
            }} else ",
            lower_name = lower_name,
            index = index,
            s = s
        )?;
    }

    writeln!(
        writer,
        "\
                {{
                    unreachable!()
                }}
            }}
        }}"
    );

    Ok(())
}
