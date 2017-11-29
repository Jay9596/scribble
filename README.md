# Scribble
Scribble is a simple CLI note taking app written in Rust. It can create, read, edit, and delete notes.

## Usage
Scribble uses some same commands as HTTP verbs.
`GET`, `POST`, `PATCH`, `PUT`, `DELETE`.

```scribble <title>```  
This commands prints the text of the note with the given title.

```scribble <command> [<args>] ```
### Commands
`scribble  help `   
Displays the help text, i.e. the usage details.

`scribble get <title>`  
This commans GETs the text of the note and prints it.  

`scribble post <title> <text>`  
This command creats a new note with the same name as title. It will overwrite a note if it already exists, use it with caution.

`scribble patch <title> <text>` OR `scribble put <title> <text>`  
This command will append the text to the note.

`scribble delete <title>` OR `scribble del <title>`  
This command will delete the note.