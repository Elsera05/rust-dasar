fn main() {
    println!("Hello, world!");
}

#[test]
fn test_main() {
    println!("Hello, test!");
}


#[test]
fn test_variable() {
    let mut name= "Achmad Rasyid Ramadhan";
    println!("hello,{}",name);
    name = "Rima Oktavianti";
    println!("hello,{}",name);    
}