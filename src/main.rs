extern crate chrono;
extern crate dirs;

use chrono::{DateTime, Datelike, Timelike, Local, TimeZone, Utc};
use std::io::{stdin, stdout, Write, self, BufReader};

//file stuff
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


//these are just temp to help me plan and keep track of what i am doing
static CONTACT_TEMPLATE: &str=
"Contact_Profile
Name:
BOD:
Age:
Birthday_Present:
Relation:
Residence:
Likes:
Dislikes:";

static EVENT_TEMPLATE: &str=
"Event_Profile
Name:
Date:
Time:
Location:
Host:
Dress_Code:
Need_To_Bring:";

static CONFIG_TEMPLATE: &str=
"RemindMe_Config
Contact_Profile: /Templates/Contact_Profile.txt
Event_Profile: /Templates/Event_Profile.txt";

static CONFIG_NAME: &str=
"Config.txt";

static PROGRAM_NAME: &str=
"RemindMe";

static TEMPLATE_FOLDER_NAME: &str=
"Templates";

//these will not all work but will work through them
static ARGUMENT_OPTIONS: &str=
"Valid Arguments:
1: add new contact
2: add new event
3: add new template
4: view all contacts
5: view all events
quit: quit";



fn main() {

    //main variables
    //directory for template files
    //directory for data files
    //directory for config file where it keeps track of all the templates
    //


    if(dirs::data_local_dir() == None){
        println!("no DIRECTORRY!!!");
        return;
    }

    //main config folder location setup
    let local_data_directory = dirs::data_local_dir().unwrap();
    let config_directory = format!("{}{}{}", local_data_directory.display(), "\\", PROGRAM_NAME);

    //main config file location
    let config_path = format!("{}{}{}", config_directory, "\\", CONFIG_NAME);
    
    let config_file = Path::new(&config_path);
    println!("Config Path: {}", &config_file.display());

    
    let template_directory = format!("{}{}{}", config_directory, "\\", TEMPLATE_FOLDER_NAME);

    
    
    std::fs::create_dir_all(&template_directory);
    //println!("{}", template_directory);
    
    //make
    if(!config_file.exists()){
        println!("config not exist");
        //create it
        let mut file = match File::create(&config_file) {
            Err(why) => panic!("couldn't create {}: {}", config_file.display(), why),
            Ok(file) => file,
        };
        match file.write_all(CONFIG_TEMPLATE.as_bytes()) {
            Err(why) => panic!("couldn't read {}: {}", config_file.display(), why),
            Ok(file) => file,
        }
        println!();
        drop(file);

    }
    
    //read file and pull
    let file = match File::open(&config_file) {
        Err(why) => panic!("couldn't open {}: {}", config_file.display(), why),
        Ok(file) => file,
    };


    //println!("!Start of FILE!");

    let mut buf_file = BufReader::new(file);

    let mut counter = 0;

    for line in buf_file.lines(){
        let line_String = line.unwrap();
        //println!("{}", line_String);

        //check if line contains a :alloc
        //if it does, then you can check it
        if (line_String.contains(":")){
            let bytes = line_String.as_bytes();
            let mut colon_index: usize = 0;
            for(i, &item) in bytes.iter().enumerate(){
                if(item ==b':'){
                    colon_index = i + 1;
                    break;
                }
            }

            let slice = (&line_String[colon_index..line_String.len()]).trim();
            //println!("Slice: {}", slice);

            //now check if template is there
            let template_path = format!("{}{}", config_directory, slice);
            let template_path = Path::new(&template_path);

            if(!template_path.exists()){
            
                let mut file = match File::create(&template_path) {
                    Err(why) => panic!("couldn't create {}: {}", template_path.display(), why),
                    Ok(file) => file,
                };

                if(line_String.contains("Contact_Profile")){

                    match file.write_all(CONTACT_TEMPLATE.as_bytes()) {
                        Err(why) => panic!("couldn't read {}: {}", template_path.display(), why), 
                        Ok(file) => file,
                    }

                } else if(line_String.contains("Event_Profile")){

                    match file.write_all(EVENT_TEMPLATE.as_bytes()) {
                        Err(why) => panic!("couldn't read {}: {}", template_path.display(), why),
                        Ok(file) => file,
                    }

                }


                println!();
                drop(file);
            
            }


        }

        counter += 1;
    }
    //println!("!End of FILE!");

    println!("{}", ARGUMENT_OPTIONS);
    let mut endLoop = false;

    while(endLoop == false){

        print!("Input request: ");
        let mut inputString = String::new();
        let _=stdout().flush();
        stdin().read_line(&mut inputString).expect("error");
        inputString = inputString.trim().to_string();

        if(inputString.eq_ignore_ascii_case("quit") == true){
            endLoop = true;
            println!("Terminating");
            break;
        }

        let mut inputInt = -1;

        if(inputString.parse::<u32>().is_ok()){
            inputInt = inputString.parse::<i32>().unwrap();
            println!("Is number");
            
        } 
                
        //want to kinda make this easily changeable
        match inputInt{

            1 => println!("One"),

            2 => println!("Two"),

            3 => println!("Three"),

            _ => println!("Invalid Argument!"),

        }
       


    }
    
}