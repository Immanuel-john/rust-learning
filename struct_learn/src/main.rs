struct Foo{
    quax: i32,
    baz: String,
    fuzz: Fuzz,
}

struct Fuzz{
    z: i32,
}
fn main() {
    let a = Foo{
        quax: 10,
        baz: String::from("Hello world"),
        fuzz: Fuzz{
            z: 20
        },
    };

    println!("Foo: quax: {} baz: {}, fuz: {}", a.quax, a.baz, a.fuzz.z);
}