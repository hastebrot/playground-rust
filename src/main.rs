fn same_length(s0: &String,
               s1: &String) -> bool {
    s0.len() == s1.len()
}

fn main() {
    let source = "hello".to_owned();
    let other = "hi".to_owned();
    let other2 = "hola".to_owned();

    println!("{:?}", same_length(&source, &other));
    println!("{:?}", same_length(&source, &other2));
}
