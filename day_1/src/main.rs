fn main() {
    let mut  x:u32=32;
    for i in 1..100 {
        x=x+1000;
    }
    let  age=true;
    let legal_age=false;
    if age && legal_age {
    //     println!("He is verified......");
    // }else{
        // print!("He is not verified....")
    }
    // println!("Hello, world!");
    // println!("{}",x);
   
    let greeting=String::from("hello world");
     println!("{}",greeting);
     let a=greeting.chars().nth(2);
     println!("{:?}",a);
}
