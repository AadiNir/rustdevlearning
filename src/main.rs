use std::io;
use rand::Rng;
use std::cmp::Ordering; //a package that gives me -1,0,1 so to compare basically

fn main() {

    //integer
    // let x: u32 = 33;
    // let y: i32 = -32; 
    // // print!("this is the var x {} and y {}",x,y)
    // //rust is memory sensistivity which means that we can actually control whatever we store, so rust allows i8 till i128 which is basically integer and u8 to u128 which is basically unsighned integer again
    
    // //booleans
    // let is_male: bool = false;
    // let is_not_male: bool = true;
    // // variables are immutable
    // if is_male && is_not_male{
    //     print!("you are a male");
    // }else{
    //     print!("you are not a male");
    // }

    // //string
    // let greeting: String = String::from("hello world");
    // //as string doesnt have defenitive size

    // let greeting_index = greeting.chars().nth(1);

    // //out of the box it wont promise you char return it might be char maybe none so to even normally print it you should use a format like this below
    // match greeting_index{
    //     Some(c)=> print!("\n this is the character {}",c),
    //     None => print!("No character at index"),
    // }
    // print!("\n{}",greeting);

    // //loops
    // for i in 0..100{

    // }
    // //usually iterate over arrays, maps, strings
    // let sentence =String::from("my name is aadithya");


    // fn get_my_first_word(val: String)->String{
    //     let mut first_word = String::new();
    //     for chars in val.chars(){
    //         if chars == ' '{
    //             break;
    //         }
    //         first_word.push(chars)
    //     }

    //     return first_word;

    // }

    // print!("this is my first word{}",get_my_first_word(sentence));


    // fn add_val(val1: i32, val2: i32)->i32{
    //     return val1 +val2;
    // }

    // let ans = add_val(100, 50);

    // print!("this is the sum val {}",ans)



    loop{
        println!("Guess a number");
        let secret_number = rand::thread_rng().gen_range(1..=100); 
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        //parseint from string
        match guess.cmp(&secret_number){
            Ordering::Less=>print!("too small"),
            Ordering::Equal=>{
                print!("Gotcha");
                break
            },
            Ordering::Greater=>print!("too large")
        }
    
        println!("Your guessed string: {}",secret_number);

    }







}
