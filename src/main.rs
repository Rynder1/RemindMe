mod functions;

extern crate chrono;
extern crate dirs;

use functions::*;
use chrono::{DateTime, Datelike, Timelike, Local, TimeZone, Utc};
use std::io::{stdin, stdout, Write, self, BufReader};

//file stuff
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


//these are just temp to help me plan and keep track of what i am doing
static CONTACT_TEMPLATE: &str=
"Contact_Profile
Name: {string}
BOD: {date}
Age: {int}
Birthday_Present: {list string}
Relation: {string}
Residence: {string}
Likes: {string}
Dislikes: {string}";

static EVENT_TEMPLATE: &str=
"Event_Profile
Name: {string}
Date: {date}
Time: {time}
Location: {string}
Host: {string/person}
Dress_Code: {string}
Need_To_Bring: {string list}";


/*
RemindMe_Config
Template_Folder_Location: C:\Users\Liam\AppData\Local\RemindMe\Templates
Data_Folder_Location: C:\Users\Liam\AppData\Local\RemindMe\Data
*/

static TEMPLATE_CONFIG_TEMPLATE: &str=
"RemindMe_Config
Contact_Profile: C:/Users/Liam/AppData/Local/RemindMe/Templates/Contact_Profile.txt
Event_Profile: C:/Users/Liam/AppData/Local/RemindMe/Templates/Event_Profile.txt";


static CONFIG_TEMPLATE: &str=
"RemindMe_Config
Template_Folder_Location: C:/Users/Liam/AppData/Local/RemindMe/Templates
Data_Folder_Location: C:/Users/Liam/AppData/Local/RemindMe/Data";

static CONFIG_NAME: &str=
"Config.txt";

static PROGRAM_NAME: &str=
"RemindMe";

static TEMPLATE_FOLDER_NAME: &str=
"Templates";

static DATA_FOLDER_NAME: &str=
"Data";

//these will not all work but will work through them
static ARGUMENT_OPTIONS: &str=
"Valid Arguments:
1: add new contact
2: add new event
3: create default templates
4: add new template
5: view all contacts
6: view all events
?: see all options
quit: quit";
//view data instead of contacts or events
//have then options based on templates



fn main() {

    //main variables
    //directory for template files
    //directory for data files
    //directory for config file where it keeps track of all the templates
    
    let mut template_folder_directory: String;
    template_folder_directory = "".to_string();


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

            let address_name = (&line_String[0..colon_index]).trim();
            let slice = (&line_String[colon_index..line_String.len()]).trim();
            
            //println!("Slice: {}", slice);

            //now check if template is there

            //slice changed so that its just the full address
            //let template_path = format!("{}{}", config_directory, slice);
            let template_path = format!("{}", slice);
            let template_path = Path::new(&template_path);
            println!("Address Name: {}", address_name);
            println!("Template Path: {}", template_path.display());

            let mut is_template = false;

            if(address_name == "Template_Folder_Location:"){
                is_template = true;
                println!("Found template folder");
                template_folder_directory = format!("{}", template_path.display());
            }

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

            println!("before template");

            if(is_template == true){
                //for each file in template folder

                println!("in template");

                for template_file in std::fs::read_dir(template_path).unwrap(){
                    
                    match template_file{

                        Ok(valid_path) => 
                        {
                            println!("Found: {}", valid_path.path().display());
                            functions::template_file_handler(&valid_path.path());
                        },
                        Err(e) => println!("Found nothing {}", e),

                    }


                    
                }

                
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

        if(inputString.eq_ignore_ascii_case("?") == true){
            endLoop = false;
            println!("{}", ARGUMENT_OPTIONS);
        } else{

            
            let mut inputInt = -1;
            
            if(inputString.parse::<u32>().is_ok()){
                inputInt = inputString.parse::<i32>().unwrap();
                println!("Is number");
                
            } 
            
            //want to kinda make this easily changeable
            match inputInt{
                
                1 => println!("One"),
                
                2 => println!("Two"),
                
                3 => {
                    println!("Three");
                    //create default templates
                    //Contact_Template, Event_Template
                    //template_folder_directory;
                    let temp = format!("{}{}",&template_folder_directory, "/Contact_Template.txt");
                    let contact_template_path = Path::new(&temp);
                    println!("3 path: {}", contact_template_path.display());
                    if(!contact_template_path.exists()){
                        println!("contact Doesn't exist");
                        let mut file = match File::create(&contact_template_path) {
                            Err(why) => panic!("couldn't create {}: {}", contact_template_path.display(), why),
                            Ok(file) => file,
                        };

                        match file.write_all(CONTACT_TEMPLATE.as_bytes()) {
                            Err(why) => panic!("couldn't read {}: {}", contact_template_path.display(), why), 
                            Ok(file) => file,
                        }

                        drop(file);
                    }


                    let event_template_path = Path::new(&config_path);
                    let temp = format!("{}{}",&template_folder_directory, "/Event_Template.txt");
                    let event_template_path = Path::new(&temp);
                    println!("3 path: {}", event_template_path.display());
                    if(!event_template_path.exists()){
                        println!("contact Doesn't exist");
                        let mut file = match File::create(&event_template_path) {
                            Err(why) => panic!("couldn't create {}: {}", event_template_path.display(), why),
                            Ok(file) => file,
                        };

                        match file.write_all(EVENT_TEMPLATE.as_bytes()) {
                            Err(why) => panic!("couldn't read {}: {}", event_template_path.display(), why), 
                            Ok(file) => file,
                        }

                        drop(file);
                    }


                },
                
                4 => 
                {
                    println!("Four");
                    functions::create_template(&Path::new(&template_folder_directory));
                },
                
                5 => println!("Five"),

                6 => println!("Five"),            
                
                _ => println!("Invalid Argument!"),
                
            }
        }
       


    }
    
}