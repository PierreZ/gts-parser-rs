#[macro_use]
extern crate nom;

use nom::digit;
use std::str;

#[derive(Debug,PartialEq)]
// TODO: move to https://github.com/CleverCloud/warp10.rs/blob/master/src/data.rs#L95
pub struct GTS {
    pub timestamp: i32,
    pub classname: String,
    pub labels: String,
}

named!(parse_gts<&str, GTS>,
    do_parse!(
        timestamp: digit >>
        tag!("// ")   >>
        classname: take_until!("{") >> 
        tag!("{") >>
        labels: take_until!("}") >>
        tag!("}") >> 
        tag!(" ") >> 
        (GTS {
            timestamp: timestamp.to_string().parse::<i32>().unwrap(),
            classname: classname.to_string(),
            labels: labels.to_string()
        })
    )
);

#[test]
fn parse() {
    assert_eq!(parse_gts("21344// test{label0=val0} 42"),
               Ok(("42",
                   GTS {
                       timestamp: 21344,
                       classname: "test".to_string(),
                       labels: "label0=val0".to_string(),
                   })));
}
