# Scribble
Scribble is a simple CLI note taking app written in Rust. It can create, read, edit, and delete notes.  
> Listen to the readme [HERE](https://postcast-app.now.sh/play/aHR0cHM6Ly9yYXcuZ2l0aHVidXNlcmNvbnRlbnQuY29tL0pheTk1OTYvc2NyaWJibGUvbWFzdGVyL1JFQURNRS5tZA==)  

## Usage
Scribble uses the same commands as HTTP verbs.
`GET`, `POST`, `PATCH`, `PUT`, `DELETE`.

```scribble <title>```  
This command prints the text of the note with the given title.

```scribble <command> [<args>] ```
### Commands
`scribble  help `   
Displays the help text, i.e. the usage details.

`scribble list `  
Lists all notes in directory.

`scribble get <title>`  
This command GETs the text of the note and prints it.  

`scribble post <title> <text>`  
This command creats a new note with the same name as title. It will overwrite a note if it already exists, use it with caution.

`scribble patch <title> <text>` OR `scribble put <title> <text>`  
This command will append the text to the note.

`scribble delete <title>` OR `scribble del <title>`  
This command will delete the note.

`scribble find <title>`
This command will search for the note with the given title.