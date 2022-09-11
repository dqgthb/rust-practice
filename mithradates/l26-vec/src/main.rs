fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    // let mut my_vec = Vec::new();

    // my_vec.push(name1);
    // my_vec.push(name2);

    // println!("My cats are {:?}", my_vec);

    let my_vec = vec![name1, name2];
    println!("My cats are {:?}", my_vec);

    let my_name = String::from("Dave MacLeod");
    let my_city: String = "Seoul".into();

    println!("{}", my_city);
}
