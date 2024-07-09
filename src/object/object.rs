// type  ObjectType = String;

use std::str::FromStr;

pub trait Object {
    fn type_of(&self) -> ObjectType;
    fn inspect(self) -> String;
}

enum ObjectType {
    INTEGER,
    BOOLEN,
    NULL,
}

struct Integer {
    value: i64,
}

impl Object for Integer {
    fn inspect(self) -> String {
        self.value.to_string()
    }
    fn type_of(&self) -> ObjectType {
        ObjectType::INTEGER
    }
}

struct Boolen {
    value: bool,
}

impl Object for Boolen {
    fn inspect(self) -> String {
        self.value.to_string()
    }
    fn type_of(&self) -> ObjectType {
        ObjectType::BOOLEN
    }
}

struct Null {
    
}

impl Object for Null {
    fn inspect(self) -> String {
        String::from("null")
    }
    fn type_of(&self) -> ObjectType {
        ObjectType::NULL
    }
}
