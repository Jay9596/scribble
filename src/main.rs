use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => print_usage(),
        2 => {
            let title = &args[1];
           match &title[..] {
               "get" | "post" | "patch" | "delete" => {},
               "list" => get_list(),
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
            }else{
                print_usage()
            }
        },
        4 => {
                let title = &args[2];
                let text = &args[3];
            if &args[1] == "post" {
                println!("Posting new note {}",title);
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
        Err(why) => panic!("Error: {}", why),
        Ok(file) => file
    };
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
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
        Err(why) => panic!("Error: {}", why),
        Ok(file) => file
    };
    let _ = file.write_all(text.as_bytes()).unwrap();
}

fn delete_note(title: &str) {
    let file_name = get_file_name(title);
    let path = Path::new(file_name.as_str());
    fs::remove_file(&path).unwrap();
}

fn get_file_name(title: &str) -> String {
    let mut file_name = String::from(title);
    file_name.push_str(".txt");
    file_name
}

fn get_list() {
    println!("Notes: ");
    let directory = Path::new(".");
    let notes : Vec<std::path::PathBuf> = fs::read_dir(&directory).unwrap()
                .map(|x| x.unwrap().path())
               .collect();
    for note in notes {
        if let Some(name) = note.file_name() {
            if let Some(name_str) = name.to_str() {
                let name = String::from(name_str);
                let f_name: Vec<&str> = name.split('.').collect();
                if f_name.len() == 2 {
                    if f_name[1] == "txt" {
                        println!("- {}", f_name[0]);
                    }
                }
            } 
        }
    }
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
help                 \tPrints help\n");
}