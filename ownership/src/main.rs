fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1.clone(), m2.clone());
    println!("{} {} from main", m1, m2);
}

fn greet(g1: String, g2: String) {
    println!("{} {} from greet", g1, g2);
}
