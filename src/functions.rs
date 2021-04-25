use std::io::{self, BufReader, Write, stdout, stdin};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

struct time_type{

}

struct date_type{

}

pub fn create_template(template_folder_path: &std::path::Path){
    println!("First line of create template!");

    let mut file_output_text = "".to_string();

    print!("Enter template name: ");

    let mut name_input = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut name_input).expect("error");
    name_input = name_input.trim().to_string();

    file_output_text += name_input.as_str();
    file_output_text += "_Profile\n";
    println!("Name String: {}", name_input);

    let mut still_going = true;
    let mut save_changes = true;

    while(still_going == true){

        println!("Type Finish when you are done or quit to not save");
        print!("Enter a variable name: ");

        let mut variable_name_input = String::new();
        let _=stdout().flush();
        stdin().read_line(&mut variable_name_input).expect("error");
        variable_name_input = variable_name_input.trim().to_string();

        match variable_name_input.to_lowercase().as_str(){

            "quit" => 
            {
                still_going = false;
                save_changes = false;
            },
            "finish" => 
            {
                still_going = false;
            },

            _ => (),
        };



        //get name for variable
        
        if(still_going == true){
     
            let type_value = get_all_data_types();
            println!("Return value: {}", type_value);

            if(type_value != ""){

                
                file_output_text += &variable_name_input;
                file_output_text += ": ";
                
                
                file_output_text += "{";
                file_output_text += &type_value;
                file_output_text += "}\n";
                
                println!("File Saved");
            }
            
        }

    }
        

    if(save_changes){
        //actually create
        let new_file_path = format!("{}{}{}{}", template_folder_path.display(), "/", name_input, "_Template.txt");
        let new_file_path = Path::new(&new_file_path);
        
        let mut file = match File::create(&new_file_path) {
            Err(why) => panic!("couldn't create {}: {}", template_folder_path.display(), why),
            Ok(file) => file,
        };
        
        match file.write_all(file_output_text.as_bytes()) {
            Err(why) => panic!("couldn't read {}: {}", new_file_path.display(), why), 
            Ok(file) => file,
        }

        drop(file);
    }


    println!("Last line of create template!");
}

pub fn get_all_data_types() -> String{

    let mut returnValue = "".to_string();

    let mut valid_choice = false;
    let mut is_quitting = false;

    let mut name_input = String::new();


    while valid_choice == false{
        
        println!("Pick a variable type:\n  Int, String, Double, Bool, Date, Time \n  or type quit");
        
        print!("Type: ");
        
        let _=stdout().flush();
        stdin().read_line(&mut name_input).expect("error");
        name_input = name_input.trim().to_string().to_lowercase();
        
        valid_choice = true;
        
        match name_input.as_str(){
            
            "int" => println!("You picked Int"),
            "string" => println!("You picked String"),
            "double" => println!("You picked Double"),
            "bool" => println!("You picked Bool"),
            "date" => println!("You picked Date"),
            "time" => println!("You picked Time"),
            "quit" => 
            {
                println!("You picked Quit: '{}'", name_input);
                is_quitting = true;
                name_input = "".to_string();
            },

            _ =>
            {
                println!("Invalid choice: '{}'", name_input);
                valid_choice = false;
            }, 
            
        }

    }

    returnValue = name_input;

    return returnValue;
}


pub fn template_file_handler(file_path: &std::path::Path){
    println!("First line of template handler!");

    println!("Handelling: {}", file_path.display());

    let file = match std::fs::File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path.display(), why),
        Ok(file) => file,
    };


    //println!("!Start of FILE!");

    let mut buf_file = std::io::BufReader::new(file);

    let mut counter = 0;

    for line in buf_file.lines(){
        let line_String = line.unwrap();
        println!("Line: {}", line_String);


    }



    println!("Last line of template handler!");
}


pub fn get_data_type(line: &std::string::String){
    println!("First line of get data type!");

    let bytes = line.as_bytes();
    let mut colon_index: usize = 0;
    for(i, &item) in bytes.iter().enumerate(){
        if(item ==b':'){
            colon_index = i + 1;
            break;
        }
    }

    let slice = (&line[colon_index..line.len()]).trim();



    println!("Path: {}", slice);
    //format of lines should be name: {type}

    



    /*
    {string} = string
    {int} = int
    {double}
    {date}
    {time}


    */
    println!("Last line of get data type!");
}

