use std::collections::HashMap;
fn main() {
    // vector with no entries needs a type
    let v: Vec<i32> = Vec::new();
    // vector with entries can have type inferred.
    let v_prime = vec![1, 2, 3];
    // to add elements to vector
    let mut v_mut = Vec::new();
    // Adding item to vector.
    v_mut.push(40);
    // get doesn't cause the program to crash.
    //
    let third: Option<&i32> = v_prime.get(2);
    // If this element didn't exist it would crash at runtime.
    let second: &i32 = &v_prime[1];

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no element here"),
    }

    let mut v_prime = vec![100, 105, 110];

    for i in &mut v_prime {
        let mut string = String::from(format!("The old value was {}", i));

        *i += 50;

        let new_bit: String = format!(", the new value is {}", i);

        string.push_str(&new_bit);

        println!("{}", string);
    }
    println!("Thanks be to jesus that I could get this right.");

    let mut scores = HashMap::new();

    scores.insert("blue", 10);
    scores.insert("red", 15);

    let team_name = "red";
    // .copied()->i32 rather than a reference to the item.
    // .unwrap_or(&self: Option<i32>)->i32. If there is no index with the
    // right value it will return 0.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Generics:
    // wher you don't know the exact type of the passed value.
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // Generics aren't slow either, compiler figures out what to do with
    // Them at compile time.
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
