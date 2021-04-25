# RemindMe
A Rust Program that will help remind people of stuff they need to do and upcoming tasks.

Aim:
- To create a program that will help people keep track of their upcoming events and tasks.

Goal Functionality:
- Ability to have contacts that you can keep track of upcoming birthdays
- Ability to show all events coming up in specified period
- Ability to gifts or requirements for events that can link to contacts
- Ability to use template profile files to create contact files off of
- Ability to edit the template profile file so that when you make a new one, you have changed fields

Layout:

Current Layout may change as project progresses.

The config folder holds the main config file which looks after the location of all the other files.
- Config Folder: Appdata\Local\RemindMe
    - Templates: \Templates


The data folder holds all the folders for the different types of data being held, contacts, events, etc.
- Data Folder: Documents\RemindMe
    - Event Folder: \Events
    - Contact Folder: \Contacts
    - Same format for all other data profiles



when making a new data, it will go through the templates folder and get all the different options for types
this can be done at the start and updated on an update command or when you add in an extra template

same type of thing when going to edit. It will show the different folders in the data folder and you pick which thing you wanna edit and then pick the actual specific data piece

it will display the file and you can pick a line number to edit or go through each line and change/add


Run order may change as project progresses.

Run order of program:
 - Checks config exists
 - if it doesn't, create it otherwise read it
 - find template folder from config
 - read in all templates into arraylist probably
 - prompt user for input/action
 - Options:
    - View Config
    - Edit Config
    - Create new template
    - View Templates
    - Edit Template
    - Create data piece
    - View data pieces
    - Edit data piece
    - view calendar (upcoming dates)
    

    
Templates currently will be using a system of "variable_name: {variable_type}"

Base data templates:
 - Contact:
    - First_Name: {string}
    - Middle_Name: {string}
    - Last_Name: {string}
    - BOD: {date}
    - Age: {int}
    - Relation: {string}
    - Address: {string}
    - Likes: {string}
    - Dislikes: {string}
    - Phone_Number: {string}
    - Email: {string}


 - Event:
    - Name: {string}
    - Date: {date}
    - Time: {time}
    - Location: {string}
    - Host: {string/person}
    - Dress_Code: {string}
    - Need_To_Bring: {string}
    
