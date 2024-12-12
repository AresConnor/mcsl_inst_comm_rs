use bitcode::{Decode, Encode};
use std::collections::HashMap;
use std::sync::LazyLock;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Encode, Decode)]
pub enum Encoding {
    ASCII,
    #[default]
    UTF8,
    UTF16LE,
    UTF16BE,
    GBK,
    GB18030,
    HZ,
    BIG5_2003,
}

fn map_encoding(encoding: &Encoding) -> encoding::EncodingRef {
    match encoding {
        Encoding::ASCII => encoding::all::ASCII,
        Encoding::UTF8 => encoding::all::UTF_8,
        Encoding::UTF16LE => encoding::all::UTF_16LE,
        Encoding::UTF16BE => encoding::all::UTF_16BE,
        Encoding::GBK => encoding::all::GBK,
        Encoding::GB18030 => encoding::all::GB18030,
        Encoding::HZ => encoding::all::HZ,
        Encoding::BIG5_2003 => encoding::all::BIG5_2003,
    }
}

static STR2ENCODING_MAP: LazyLock<HashMap<&'static str, Encoding>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("ascii", Encoding::ASCII);
    map.insert("utf-8", Encoding::UTF8);
    map.insert("utf-16le", Encoding::UTF16LE);
    map.insert("utf-16be", Encoding::UTF16BE);
    map.insert("gbk", Encoding::GBK);
    map.insert("gb18030", Encoding::GB18030);
    map.insert("hz", Encoding::HZ);
    map.insert("big5-2003", Encoding::BIG5_2003);
    map
});

impl Encoding {
    pub fn get(&self) -> encoding::EncodingRef {
        map_encoding(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_encodings() -> Vec<Encoding> {
        Vec::from_iter(STR2ENCODING_MAP.values().cloned())
    }

    #[test]
    fn str_to_encoding_map_test() {
        for encoding in get_encodings() {
            let encoding_name = encoding.get().name();
            assert_eq!(&encoding, STR2ENCODING_MAP.get(encoding_name).unwrap());
        }
    }
}
