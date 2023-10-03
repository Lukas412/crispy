fn main() {
    let program = ();
}

enum Value<'a> {
    Number(u8),
    String(&'a str)
}

struct Data<T> {
    values: Vec<T>
}
