use std::io::Write;



pub fn SolutionEx2() {

    let testStr = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
   
    let mut input_string = String::new();
    
    print!("---> Input word(s) to search: ");
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
    let len = input_string.trim_end_matches(&['\r', '\n'][..]).len();
    input_string.truncate(len);

    let count = count_char(input_string.to_lowercase().to_owned(), testStr.to_lowercase().as_str());
    println!("You have {}  '{}' in slice: \n {}",count,input_string,testStr);
}

//Solution for exercise #2
fn count_char(input:String,slice: &str) -> usize {
    let count : usize = slice.matches(&input.as_str().trim()).count();
    return count;
}
