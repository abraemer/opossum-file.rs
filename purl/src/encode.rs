use percent_encoding::{AsciiSet, CONTROLS};

const PURL_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'/')
    .add(b'@')
    .add(b'?')
    .add(b'#')
    .add(b'%');

pub fn encode(s: &str) -> String {
    percent_encoding::percent_encode(s.as_bytes(), PURL_ENCODE_SET).to_string()
}

pub fn encode_qualifier_value(s: &str) -> String {
    let encoded = encode(s);
    encoded.replace("%3A", ":").replace("%2F", "/")
}

pub fn decode(s: &str) -> String {
    percent_encoding::percent_decode(s.as_bytes())
        .decode_utf8_lossy()
        .into_owned()
}
