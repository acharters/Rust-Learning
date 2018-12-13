
struct MyStruct {
    value: bool,
    name: String
}

enum MyEnum {
    One,
    Two((i32, String)),
    Three(MyStruct)
}

fn main() {
    let x = MyStruct {
        value: true,
        name: String::from("username")
    };
    let e1 = MyEnum::One;
    let t = (9, String::from("Tuple str"));
    let e2 = MyEnum::Two(t);
    let e3 = MyEnum::Three(x);
    //println!("{} | {}", e2.Two.2, e3.Three.name); code doen't work
    let tuple_str = match e2 {
        MyEnum::One => String::from("ERROR"),
        MyEnum::Two((_, y)) => y,
        MyEnum::Three(_) => String::from("ERROR")
    };
    let username_str = match e3 {
        MyEnum::One => String::from("ERROR"),
        MyEnum::Two(_) => String::from("ERROR"),
        MyEnum::Three(x) => x.name
    };
    //println!("{} | {}", tuple_str, username_str);
    
}
