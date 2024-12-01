use crate::aoc::*;

pub fn first_problem(path: &Path){
    let contents = get_file_content(&path);
    let numbers = get_numbers(&contents);
    let result = add_string_numbers(&numbers);
    println!("The resulting sum is: {}", result);
}

pub fn second_problem(path: &Path){
    let mut contents = get_file_content(&path);
    convert_number_strings(&mut contents);
    let numbers = get_numbers(&contents);
    let result = add_string_numbers(&numbers);
    println!("The resulting sum is: {}", result);
}

mod tests {
    
    use crate::aoc::*;

    #[test]
    fn file_read(){
        let text = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        let path = Path::new("./data/day1_example.txt");
        let result = get_file_content(&path);
        assert_eq!(text, result);
    }

    #[test]
    fn number_parsing(){
        let path = Path::new("./data/day1_example.txt");
        let contents = get_file_content(&path);
        let result = get_numbers(&contents);
        assert_eq!(vec!["12", "38", "15", "77"], result);
    }

    #[test]
    fn number_adding(){
        let path = Path::new("./data/day1_example.txt");
        let contents = get_file_content(&path);
        let numbers = get_numbers(&contents);
        let result = add_string_numbers(&numbers);
        assert_eq!(result, 142);
    }

    #[test]
    fn number_conversion(){
        let path = Path::new("./data/day1_example2.txt");
        let mut result = get_file_content(&path);
        convert_number_strings(&mut result);
        assert_eq!(String::from("219\n8wo3\nabc123xyz\nx2ne34\n49eight72\nz1ight234\n7pqrst6teen\n"), result);
    }

    #[test]
    fn new_number_adding(){
        let path = Path::new("./data/day1_example2.txt");
        let mut contents = get_file_content(&path);
        convert_number_strings(&mut contents);
        let numbers = get_numbers(&contents);
        let result = add_string_numbers(&numbers);
        assert_eq!(result, 281);
    }
}
