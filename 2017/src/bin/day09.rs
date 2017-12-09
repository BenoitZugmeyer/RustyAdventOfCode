#[macro_use]
extern crate nom;

use std::io::stdin;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum Stream {
    Group(Vec<Stream>),
    Garbage(String),
}

impl Stream {
    fn score(&self, parent_score: u32) -> u32 {
        match *self {
            Stream::Group(ref children) => {
                1 + parent_score +
                    children
                        .iter()
                        .map(|c| c.score(parent_score + 1))
                        .sum::<u32>()
            }
            Stream::Garbage(_) => 0,
        }
    }
    fn characters(&self) -> usize {
        match *self {
            Stream::Group(ref children) => children.iter().map(|c| c.characters()).sum::<usize>(),
            Stream::Garbage(ref s) => s.len(),
        }
    }
}

named!(
    parse_group<Stream>,
    do_parse!(
        children: delimited!(
            tag!("{"),
            separated_list!(tag!(","), parse_stream),
            tag!("}")
        ) >>
        (Stream::Group(children))
    )
);

named!(
    parse_garbage<Stream>,
    alt!(
        map!(tag!("<>"), |_| Stream::Garbage(String::from(""))) |
        do_parse!(
            tag!("<") >>
            content:
                escaped_transform!(
                    take_until_either!(">!"),
                    '!',
                    map!(take!(1), |a| &b"")
                )
            >>
            tag!(">") >>
            (Stream::Garbage(String::from_utf8_lossy(&content).to_string()))
        )
    )
);

named!(
    parse_stream<Stream>,
    alt!(parse_group | parse_garbage)
);




fn main() {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).expect(
        "Failed to read stdin",
    );
    let (_, stream) = parse_stream(&input).unwrap();

    println!("Part 1: {}", stream.score(0));
    println!("Part 2: {}", stream.characters());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stream_test() {
        assert_eq!(
            parse_stream(&b"<a>"[..]),
            nom::IResult::Done(&b""[..], Stream::Garbage(String::from("a")))
        );
        assert_eq!(
            parse_stream(&b"<>"[..]),
            nom::IResult::Done(&b""[..], Stream::Garbage(String::from("")))
        );
        assert_eq!(
            parse_stream(&b"<!>>"[..]),
            nom::IResult::Done(&b""[..], Stream::Garbage(String::from("")))
        );
    }
}
