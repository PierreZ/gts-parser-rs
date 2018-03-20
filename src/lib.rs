#[macro_use]
extern crate nom;
extern crate time;
extern crate warp10;

use nom::digit;
use std::str;

named!(parse_gts<&str, warp10::Data>,
    do_parse!(
        // TODO: put actual timestamp and test it somehow
        timestamp: alt!( digit | value!("1")) >>
        tag!("/")   >>
        // TODO: handle geo
        // /48.0:-4.5/
        tag!("/ ")   >>
        classname: take_until!("{") >>
        tag!("{") >>
        // TODO: handle labels
        // label0=val0,label1=val1 into Vec
        labels: take_until!("}") >>
        tag!("}") >>
        tag!(" ") >>
        value: take_until_and_consume!("\n") >>
        (warp10::Data::new(
                time::Timespec::new(0, timestamp.to_string().parse::<i32>().unwrap() * 1000),
                None,
                classname.to_string(),
                Vec::new(),
                warp10::Value::Int(value.to_string().parse::<i32>().unwrap()),
        ))
    )
);

#[test]
fn parse() {
    println!("{:?}", parse_gts("21344// test{label0=val0} 42\n"))
}
