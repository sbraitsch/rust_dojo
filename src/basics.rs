use std::num::ParseIntError;

pub fn introduce() {

    // 8 types in total. 8, 16, 32 and 64 bit. signed and unsigned
    let a_number: i8 = 127;
    let bigger_number: u8 = 255;
    println!("My numbers are: {} and {}\n", a_number, bigger_number);


    // a managed String (vec of chars) on the heap. can be modified
    let string_object: String = String::from("My String");
    // an immutable string. hardcoded into the binary
    let string_literal: &'static str = "My &str";
    // an immutable string slice, looking into our String on the heap
    // does not take up additional memory
    let string_slice: &str = &string_object[0..2];
    println!("My strings are: {}, {} and {}\n", string_object, string_literal, string_slice);


    // vector -> collection of elements
    // vec! macro allows for inserting values on instantiation
    let a_vec: Vec<&str> = vec!["have", "you", "fed", "ferris", "yet?"];
    // if you want to create an empty vec and fill it, you need it to be mutable
    let mut other_vec: Vec<i32> = Vec::new();
    other_vec.push(32);
    // structs that derive the Debug trait can be printed using :? in the template
    println!("My vec contains: {:?}\n", a_vec);


    // you can apply anonymous functions (closures) to all elements of a vector
    // very similar to lambdas, but they also capture surrounding variables
    let mapped_vec: Vec<String> = a_vec.iter().map(|element| element.to_uppercase()).collect();
    println!("My loud vec contains: {:?}\n", mapped_vec);


    // result is an enum with two variants. ok and err.
    let ok_result: Result<i32, ParseIntError> = "32".parse();
    let err_result: Result<i32, ParseIntError> = "NAN".parse();

    // you can .unwrap results to use the contained value
    ok_result.unwrap();
    // unwrapping an err will cause a panic (program will crash)
    // we can safely handle results (and options) using match
    match err_result {
        Ok(value) => println!("My Ok Result contained: {}", value),
        Err(error) => println!("I got this Error in my Result: {:?}", error)
    }
    
    // ferris is owned by the main function
    let ferris = Crab {
        name : "Ferris 🦀".to_string(),
        age: 80,
        height: 0.12,
    };

    // rust variables are immutable by default
    // ferris.name = "asdasd".to_string();

    // ownership of ferris is given to the consume function
    // consume(ferris);
    // ferris' ownership has been moved to consume and was dropped at this point
    // the compiler won't let us use ferris from this point
    borrow(&ferris);

    match try_to_ride_wheel(&ferris) {
        Ok(value) => println!("{}\n", value),
        Err(error) => println!("{:?}\n", error)
    }

    // ______________________OPTIONAL_______________________
/*     let string_one = String::from("i live longer");
    let result;

    {
        let string_two = String::from("i live shorter");
        result = lifetime_example(&string_one, string_two.as_str());
    }
    
    println!("{}", result); */
    
}

// similar to implementing an interface with a default implementation
#[derive(Debug)]
struct Crab {
    name: String,
    age: u8,
    height: f32
}

impl Crab {
    fn is_tall_enough(&self) -> bool {
        self.height >= 0.15
    }
    fn is_old_enough(&self) -> bool {
        self.age > 18
    }
    fn end_it(self) {
        // crab will die.
    }
}

fn consume(crab: Crab) {
    println!("Oh shit, I ate: {:?}\n", crab);
    // the consume function goes out of scope and drops the crab
}

fn borrow(crab: &Crab) {
    println!("Just borrowing: {:?}\n", crab);

}

fn try_to_ride_wheel(crab: &Crab) -> Result<&str, String> {
    
    /*
    
    if crab.is_tall_enough() && crab.is_old_enough() {
        Ok("Weeeeeee")
    } else {
        Err(format!("{} is crying because he's too small to ride the ferris wheel.", crab.name))
    }

    */
    
    match crab.is_tall_enough() && crab.is_old_enough() {
        true => Ok("Weeeeeee"),
        false => Err(format!("{} is crying because he's too small to ride the ferris wheel.", crab.name))
    }
}

fn lifetime_example<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
