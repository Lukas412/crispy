use std::collections::HashMap;

fn main() {
    let program = ();
}

fn parse_to_programm(programm: &mut Programm, input: &str) {}

fn parse_and_evaluate(programm: &Programm, input: &str) -> GenericValue {}

struct Programm {
    functions: HashMap<Box<str>, fn(List<GenericValue>) -> GenericValue>,
}

enum GenericValue<'a> {
    Symbol(&'a str),
    Number(u8),
    Text(&'a str),
    List(List<Self>),
    FunctionCall(FunctionCall<'a, Self>),
}

struct List<T> {
    values: Vec<T>,
}

struct FunctionCall<'a, Arguments> {
    name: &'a str,
    arguments: List<Arguments>,
}
