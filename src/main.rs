use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => print_usage(),
        2 => {
            let title = &args[1];
           match &title[..] {
               "get" | "post" | "patch" | "delete" => {},
               "list" => print_list(),
               "help" => print_usage(),
               _ => {
                   println!("Fetching note :{}", title);
                   get_note(title);
               },
           }
        }
        3 => {
            if &args[1] == "get"{
                println!("Fetching note :{}", &args[2]);
                get_note(&args[2]);
            }else if &args[1] == "delete" || &args[1] == "del" {
                println!("Deleting {}",&args[2]);
                delete_note(&args[2]);
            }else if &args[1] == "find" {
                find_note(&args[2])
            }else{
                print_usage()
            }
        },
        4 => {
                let title = &args[2];
                let text = &args[3];
            if &args[1] == "post" {
                println!("Posting new note :{}",title);
                post_note(title, text);
            } else if &args[1] == "patch" || &args[1] == "put" {
                println!("Patching note {}", title);
                patch_note(title, text);
            } else {
                print_usage();
            }
        },
        _ => {}
    }
}

fn get_note(title: &str) {
    let file_name = get_file_name(title);
    let path = Path::new(file_name.as_str());
    // println!("path {}", path.display());
    let mut file = match File::open(&path) {
        Err(why) => {
            println!("Error: {}", why);
            process::exit(1);
        },
        Ok(file) => file
    };
    let mut text = String::new();
    match file.read_to_string(&mut text){
        Ok(_) => {},
        Err(why) => {
            println!("Error: {}", why);
            process::exit(1);
        },
    };
    println!("Text:\n{}", text);
}

fn post_note(title: &str, text: &str) {
    let file_name = get_file_name(title);
    let path = Path::new(file_name.as_str());
    println!("path {}", path.display());
    let mut file = match OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(&path){
        Ok(file) => file,
        Err(why) => panic!("Could not create file {}", why),
    };
    let _ = file.write(text.as_bytes()).unwrap();

}

fn patch_note(title: &str, text: &str) {
    let file_name = get_file_name(title);
    let path = Path::new(file_name.as_str());
    // println!("path {}", path.display());
    let mut file = match OpenOptions::new()
                            .read(true)
                            .append(true)
                            .open(&path) {
        Err(why) => {
            println!("Error: {}", why);
            process::exit(1);
        },
        Ok(file) => file
    };
    let _ = file.write_all(text.as_bytes()).unwrap();
}

fn delete_note(title: &str) {
    let file_name = get_file_name(title);
    let path = Path::new(file_name.as_str());
    match fs::remove_file(&path){
        Ok(_) => {},
        Err(why) => {
            println!("Error: {}", why);
            process::exit(1);
        },
    }
}

fn find_note(name: &str) {
    let mut flag = false;
    let list = get_list();
    for note in list {
        if note == name {
            flag = true;
            println!("Found note: {}", note);
        } 
    }
    if !flag {
        println!("No notes found with title '{}'", name);
        println!("Use `list` command to see available notes");
    }
}

fn get_file_name(title: &str) -> String {
    let mut file_name = String::from(title);
    file_name.push_str(".txt");
    file_name
}

fn print_list() {
    println!("Notes: ");
    let list = get_list();
    for name in list {
        println!("- {}", name);
    }
}

fn get_list() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    let directory = Path::new(".");
    let files: Vec<std::path::PathBuf> = fs::read_dir(&directory)
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    for file in files.clone() {
        if let Some(name) = file.file_name().unwrap().to_str() {
            let f_name = String::from(name);
            let name: Vec<&str> = f_name.split('.').collect();
            if name.len() == 2 {
                if name[1] == "txt" {
                    list.push(String::from(name[0]));
                }
            }
        }
    }
    list
}

fn print_usage() {
    println!("Usage: 
scribble <title>\tPrints the note.
scribble list\tList all notes in directory.

scribble <command> [<args>]\n
Commands:
post   <title> <text>\tWrites the text in the note.
get    <title>       \tPrints the note.
patch  <title> <text>\tUpdates <text> of note.
delete <title>       \tDelete the note.
find <title>         \tSearches for the note with given title.
help                 \tPrints help\n");
}