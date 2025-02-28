struct signup{
    user:String,
    email:String,
    age:u32,
    mobile_number:u32,
}

fn main() {
    let user1=signup{
        user:String::from("Preetham Reddy"),
        email:String::from("someone@gmail.com"),
        age:29,
        mobile_number:999999999
    };
    println!(" {} ,has been signed up with email {} ,{} ,{} ",user1.user,user1.email,user1.age,user1.mobile_number);
    // let user2=signup{
    //     user:String::from("something"),
    //     email:user1.email,
    //     age:user1.age,
    //     mobile_number:user1.mobile_number
    // };
    // println!(" {} ,has been signed up with email {} ,{} ,{} ",user2.user,user2.email,user2.age,user2.mobile_number);

    let mut user3=signup{
        user:String::from("something and someone"),
        ..user1
    };
    user3.user=String::from("workafell");
    println!(" {} ,has been signed up with email {} ,{} ,{} ",user3.user,user3.email,user3.age,user3.mobile_number);

}

fn update_str(s:& mut String){
    s.push_str("world");
    println!("{}",s);
}

fn printing(SomeString:String)->String{
    // println!("{}",SomeString);
    return SomeString;
}



fn get_first_word(sentance: String)->String{
    let mut first_word=String::from("");
    for char in sentance.chars(){
        first_word.push_str(char.to_string().as_str());
        if char==' ' {
            break;
        }
    }
    return first_word;
}


fn get_even_numbers(number:i32) {
    if number%2==0{
        println!("{} is even",number);
    }
}


fn stack_fn(){
    let a=32;
    let b=45;
    let c=a+b;
    println!("The sum of {} and {} is {}",a,b,c);
}


fn heap_fn(){
    let s1=String::from("I am s1");
    let s2=String::from("I am s2");
    let combined=format!("{} {}",s1,s2);
    println!("{}",combined);

}

fn sum(a:i32,b:i32)->i32{
    let c=a+b;
    return c;
}

// fn update_str(){
//     let mut s4=String::from("In am s4");
//     println!("Before updating the String {} ",s4);
//     for i in 0..100{
//         s4.push_str("I am updated s4");
//         println!("length: {} , capacity: {} , pointer: {:p}",s4.len(),s4.capacity(),s4.as_ptr());
//     }
    

// }


// // println!("Hello, world!");
//     // let sentance=String::from("Preetham is a good boy.....");
//     // let name=get_first_word(sentance);
//     // println!("{}",name);
//     // let number=100;
//     // let is_even=get_even_numbers(number);
//     // let mut react_founder=String::from("Preetham ");
//     // react_founder.push_str("Reddy");
//     // println!("MY name is {}", react_founder);
//     // stack_fn();
//     // heap_fn();
//     // update_str();
 // let x=1;
    // let y=4;
    // println!("{}",sum(x,y));
     // let  my_str=String::from("I am my string.......");
    // let s1=my_str;
    //  println!("{}",s1);
    // let mut my_string=String::from("Hello ");
    // let s1=& mut my_string;
    // // let s2=& mut my_string;
    // //  println!("in main funtion {}",& mut my_string);
    // update_str(s1);
    // // update_str(s2);


