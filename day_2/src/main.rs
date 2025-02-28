

fn main() {
    println!("Hello, world!");
    let sentance=String::from("Preetham is a good boy.....");
    let name=get_first_word(sentance);
    println!("{}",name);
    let number=100;
    let is_even=get_even_numbers(number);
    let mut react_founder=String::from("Preetham ");
    react_founder.push_str("Reddy");
    println!("MY name is {}", react_founder);
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





