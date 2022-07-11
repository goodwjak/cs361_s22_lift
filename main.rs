/*
 * Author: Jake Goodwin
 * Date: 2022
 * Description: A workout cli program.
 */

//Imports and crates
use rusqlite::{params, Connection, Result};
use std::env;


//Global stuff
const DB_NAME: &str = "lift_data.db";
const PGRM_NAME: &str = "LIFT_CLI";
const AUTHOR: &str = "Jake Goodwin";


/*
 * #################################
 * Data Strucs
 * #################################
 */


//Movments have a name and id, maybe later they will have other info
//The derive(debug) is an attribute on the struct, we're asking rustc
//to autogen a debug trait for stuff.
#[derive(Debug)]
struct Movement {
    id: i32,
    name: String,
    is_upper: bool,
    require_weight: bool,
}


//Sets consist of a movement, reps, timings, repitition and weight.
//Also a date, because we need to track when it was done.
#[derive(Debug)]
struct Set {
    id: i32,
    weight: i32,
    is_db: bool, //DB is for dumbell in this case, not database.
    reps: i32,
    num_sets: i32,
    unix_time: u64, //Might swap out for another struct at some point?
}

/*
 * Thinking about adding a workout struct.
 * It would be a list or array of sets with movements.
 * Also I should probably make a section for tags in the movments.
 * This might help with finding them later.
 */

#[derive(Debug)]
struct Workout {
    id: i32,
    name: String,
    focus: String, //connective tissue, strength, power, hypertrophy ect.
    Description: String,
    date: u64,
}



/*
 * #################################
 * FUNCTIONS
 * #################################
 */


/*
 * Input: None
 * Output: text
 * Description: Prints out the CLI help documentation
 */
fn show_help() {
    println!("Program Name: {}", PGRM_NAME);
    println!("DataBase File: {}", DB_NAME);
    println!("Author: {}", AUTHOR);
    println!("\n\nDescription:\nLift is a command line interface program
        for keeping track of diffent kinds of movements and lift data.\n\n
        OPTIONS:\n
        add: usage: --> lift add <name> <is_upper true/false> <require_weight true/false>\n
        Adds a movement to the database for future use.\n\n
        movements: usage --> lift movements\n
        Shows all the movements in the database.\n\n
        del: usage: --> lift del move <name>\n
        Removes a movements from the database using it's name.\n\n
        undo: usage --> lift undo\n
        reverts the database back before the lastrun command.\n\n");
}


/*
 * Input: Movement struct
 * Output: Movement into database table.
 * Description: Trys to insert a movements into a database table.
 */
fn add_movement(new_move: Movement) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    conn.execute("create table if not exists movements
        (id integer primary key,
         name text not null unique,
         is_upper integer,
         require_weight integer)", [],)?;

    //Now we add the movement to the table.
    conn.execute("INSERT INTO movements 
        (id, name, is_upper, require_weight)
        VALUES (?1, ?2, ?3, ?4)",
        params![new_move.id,new_move.name, new_move.is_upper, new_move.require_weight],)?;
    Ok(())
}

/*
 * Input: Movement key
 * Output: none
 * Description: Deletes a movement from the database.
 */
fn del_movement(move_name: &String) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;   
    //Now we del the movement to the table.
    conn.execute("DELETE FROM movements WHERE name=(?1)", params![move_name],)?;
    Ok(())
}

/*
 * Input: none
 * Output: none
 * Description: shows movements from the database.
 */
fn show_all_movements() -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    let mut stmt = conn.prepare("SELECT * FROM movements")?;
    
    //Map to struct and query for each entry
    let move_iter = stmt.query_map([], |row| {
        Ok(Movement {id: row.get(0)?, 
            name: row.get(1)?, 
            is_upper: row.get(2)?, 
            require_weight: row.get(3)?, })},)?;

    //Now we iterate through all movements
    for m in move_iter {
        println!("{:?}", m.unwrap());
    }
    
    Ok(())
}


/*
 * Input: string
 * Output: bool value
 * Description: figures out if a string is True or false
 */
fn string_to_bool(in_str: &str) -> bool {
    let clean_str = in_str.trim().to_lowercase();
    if clean_str == "1" || clean_str == "true" || clean_str == "yes" || clean_str == "T" {
        return true;
    }
    else {
        return false;
    }
}

/*
 * Input: None
 * Output: old database file
 * Description: imports the old database as current to "undo" a change.
 */
fn undo_database() {
    println!("undo_database()");
}


/*
 * Input: Command line arguments 
 * Output: Text to screen
 * Description: Parses arguments to create, update, delete workout lift data.
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Passed args:{:?}", args);

    if args[1] == "add" && args[2] == "move" {
        println!("add_movement()");
        
        //create movment struct
        let movement: Movement = Movement {
            id: 0001,
            name: args[3].to_string(),
            is_upper: string_to_bool(&args[4]),
            require_weight: string_to_bool(&args[5]),
            };
        println!("movement struct: {:?}", movement);
        //add movement to database
        add_movement(movement);
    }
    if args[1] == "del" && args[2] == "move" {
        println!("del_movement()");
        del_movement(&args[3]);
    }
    if args[1] == "movements" {
        println!("show_all_movements()");
        show_all_movements();
    }
    if args[1] == "help" {
        show_help();
    }
}





