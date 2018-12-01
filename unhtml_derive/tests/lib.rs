#![feature(custom_attribute)]

extern crate unhtml;
extern crate unhtml_util;
use unhtml::unhtml;
use unhtml_util::*;

#[cfg(test)]
mod test;

// default test case
#[unhtml]
struct DefaultUser {
    // invoke String::from
    #[html(selector = "#non-exist", default = "Hexilee")]
    name: String,

    // invoke u8::from<u8>
    #[html(default = 20)]
    age: u8,

    // invoke i64::from_str
    #[html(default = "-1000")]
    assets: i64,

    #[html(default = true)]
    like_lemon: bool,
}

#[unhtml]
#[html(selector = "#test")]
struct SingleUser {
    #[html(selector = "p:nth-child(1)", attr = "value")]
    name: String,

    #[html(selector = "p:nth-child(2)", attr = "value")]
    age: u8,

    #[html(selector = "p:nth-child(3)", attr = "value")]
    like_lemon: bool,
}

#[unhtml]
struct TestUser {
    #[html(selector = "p:nth-child(1)", attr = "value")]
    name: String,

    #[html(selector = "p:nth-child(2)", attr = "value")]
    age: u8,

    #[html(selector = "p:nth-child(3)", attr = "value")]
    like_lemon: bool,
}

#[unhtml]
#[html(selector = "#test")]
struct TestUsers {
    #[html(selector = "div")]
    users: Vec<TestUser>,
}