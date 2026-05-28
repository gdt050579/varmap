use varmap::*;

#[derive(VarMapValue, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let mut m = StrVarMap::new();
    m.set("object.name", "John");
    m.set("object.surname", "Doe");
    m.set("object.age", 33);
    m.set("object.email", "john.doe@example.com");
    m.set("object.visible", true);
    m.set("object.point", Point { x: 10, y: 20 });

    m.update::<i32>("object.age", |age| *age += 1);

    println!("name   : {}", m.get_str("object.name").unwrap());
    println!("surname: {}", m.get_str("object.surname").unwrap());
    println!("age    : {}", m.get_i32("object.age").unwrap());
    println!("email  : {}", m.get_str("object.email").unwrap());
    println!("visible: {}", m.get_bool("object.visible").unwrap());
    let point = m.get::<Point>("object.point").unwrap();
    println!("point  : x={}, y={}", point.x, point.y);
    m.update::<Point>("object.point", |point| point.x += 1);
    let point2 = m.get::<Point>("object.point").unwrap();
    println!("point2 : x={}, y={}", point2.x, point2.y);
}
