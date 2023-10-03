fn main() {
    let program = ();
}

enum GenericValue<'a> {
    Symbol(&'a str),
    Number(u8),
    Text(&'a str),
    List(List<Self>),
    FunctionCall(FunctionCall<'a, Self>)
}

struct FunctionCall<'a, Arguments> {
    name: &'a str,
    arguments: List<Arguments>
}

struct List<T> {
    values: Vec<T>
}
