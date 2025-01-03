use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
        }
    });

    for i in 1..5 {
        println!("Main thread: {}", i);
    }

    handle.join().unwrap();
}




// #[test]
// fn test_main() {
//     println!("Hello, test!");
// }


// #[test]
// fn test_variable() {
//     let mut name= "Achmad Rasyid Ramadhan";
//     println!("hello,{}",name);
//     name = "Rima Oktavianti";
//     println!("hello,{}",name);    
// }


// #[test]
// fn shadowing (){
//     let name = "mamads";
//     println!("hello,{}",name);
//     let name = 10;
//     println!("hello,{}",name);
// }


// // arc belajar variabel

// #[test]
// fn nama_main (){
//     let name = "Achmad Rasyid ramadhan ";
//     println!("hello {}",name);
// }



// #[test]
// fn test_mutable() {
//     let mut age = 25;
//     println!("your age is ,{}",age);
//     age = 30;
//     println!("your age is ,{}",age);    
// }


// #[test]
// fn test_f64(){
//     let x = 2.0;
//     let y: f32 = 3.0;
//     println!("x = {}, y = {}",x,y);
// }



// // fn greet(){
// //     println!("hello world",)
// // }

// // fn main() {
// //     println!("Hello, rust");
// //     greet();
// // }


// #[test]
// fn jumlah(){
//     let a = 10;
//     let b = 5;
//     let c = a - b;
//     println! ("hasilnya adalah {}",c);
// }



// #[test]
// fn tests_f64(){
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
//     let x_as_f32 = x as f32;
//     println!("x = {}, y = {}", x_as_f32, y);
// }



// fn greet(name: String) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     let name = String::from("Achmad");
//     greet(name);
// }
