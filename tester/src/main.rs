use varmap::*;


fn main() {
    let mut m = StrVarMap::new();
    m.set("object.name", "John");
    m.set("object.surname", "Doe");
    m.set("object.age", 33);
    m.set("object.email", "john.doe@example.com");
    m.set("object.visible", true);

    println!("name   : {}", m.get_str("object.name").unwrap());
    println!("surname: {}", m.get_str("object.surname").unwrap());
    println!("age    : {}", m.get_i32("object.age").unwrap());
    println!("email  : {}", m.get_str("object.email").unwrap());
    println!("visible: {}", m.get_bool("object.visible").unwrap());
}
