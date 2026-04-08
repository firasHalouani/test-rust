use std::fs::File;
use std::io::Read;

fn main() {
    // ❌ unwrap dangereux (peut crash)
    let f = File::open("test.txt").unwrap();

    // ❌ variable inutilement mutable
    let mut x = 5;

    // ❌ clone inutile
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}", s2);

    // ❌ conversion inutile
    let y = 10 as i32;

    // ❌ boucle inefficace
    let v = vec![1, 2, 3];
    for i in 0..v.len() {
        println!("{}", v[i]);
    }

    // ❌ if inutile
    if true == true {
        println!("Always true");
    }

    // ❌ variable non utilisée
    let unused_var = 42;

    // ❌ lecture fichier sans gestion d’erreur
    let mut content = String::new();
    let mut file = File::open("data.txt").unwrap();
    file.read_to_string(&mut content).unwrap();

    println!("{}", content);
}
