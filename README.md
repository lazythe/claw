<h1 align="center">
  <br>
  <img src="https://raw.githubusercontent.com/lazythe/claw/master/logo.png" alt="CLAW" width="200">
  <br>
  CLAW
  <br>
</h1>

<h4 align="center">A Unix shell written in Rust.</h4>

# Currently Implemented

### Basic Functionality 

* Tab completion for commands and files
* History
* Colorization
* Command redirection
* Pipe
* Background processes
* Directory stack operations

### Basic Commands

* Directory traversal using ```cd /path```
* Clear terminal using ```clear```
* Exit shell using ```exit```
* List commands using ```help```

### Directory Stack Operations

* Show current stack ```dirs```
* Save current dir and go to path ```pushd /path```
* Pop last directory from stack ```popd```

### Coming Soon

* History search using ```history | grep <pattern>```
* Search for files using ```find /path -name <pattern>```
* Search for directories using ```find /path -type d```
