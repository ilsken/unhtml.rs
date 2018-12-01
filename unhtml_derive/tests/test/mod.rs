use super::{DefaultUser, SingleUser, TestUsers};
use unhtml_util::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_default_value() {
    let user: DefaultUser = DefaultUser::from_str("").unwrap();
    assert_eq!("Hexilee".to_string(), user.name);
    assert_eq!(20, user.age);
    assert_eq!(-1000, user.assets);
    assert!(user.like_lemon);
}

#[test]
fn test_single_user() {
    let user: SingleUser = SingleUser::from_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <p>Hexilee</p>
            <p>20</p>
            <p>true</p>
        </div>
    </div>
</body>
</html>"#).unwrap();
    assert_eq!("Hexilee".to_string(), user.name);
    assert_eq!(20, user.age);
    assert!(user.like_lemon);
}

#[test]
fn test_users_parse() {
    let users: TestUsers = TestUsers::from_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <p>Hexilee</p>
            <p>20</p>
            <p>true</p>
        </div>
        <div>
            <p>BigBrother</p>
            <p>21</p>
            <p>false</p>
        </div>
    </div>
</body>
</html>"#).unwrap();
    let hexilee = users[0];
    let big_brother = users[1];
    assert_eq!("Hexilee".to_string(), hexilee.name);
    assert_eq!(20, hexilee.age);
    assert!(hexilee.like_lemon);
}