# <p align="center">Rust-for-all</p>

hey guys my Name is <b>Dhanasai</b> and from today onwards we are going to learn about the rust


so basically rust is a newly emerged language and open source language that is mainly used for writing the systems software like writing operating systems, device drivers and it is also used to develop game engines.it is mainly used for the code saftey and performence


so in today's session we are going to learn about three topics


### 1. Learn how to install and run rust in your computer
### 2. Learn how to write your first ever hello world program in rust
### 3. Learn how to manage the bigger project in rust using cargo


## Install rust on your system:

The first step is to install Rust. We’ll download Rust through rustup, a command line tool for managing Rust versions and associated tools.You’ll need an internet connection for the download.

<b>Installing rustup on Linux or macOS</b>

If you’re using Linux or macOS, open a terminal and enter the following command:

`$ curl https://sh.rustup.rs -sSf | sh`

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

`Rust is installed now. Great!`

<b>Installing rustup on Windows</b>

On Windows, go to https://www.rust-lang.org/tools/install/ and follow the instructions for installing Rust. After that rust will autmatically installed on your Windows system.To confirm the installation open up the cmd and type in following command:

`rustc -V`

<b>Updating and Uninstalling</b>

After you’ve installed Rust via rustup, updating to the latest version is easy. From your shell, run the following update script:

`$ rustup update`

To uninstall Rust and rustup, run the following uninstall script from your shell:

`$ rustup self uninstall`


<b>Local Documentation</b>

The installer also includes a copy of the documentation locally, so you can read it offline. Run the `rustup doc` to open the local documentation in your browser.


## Writing Hello World!

Now we have succesfully installed rust and now we are going to write our first program in rust.It is like a tradition to start learning a programming language with the hello world program.

so to write a program open up your favorite text editor(i'm using VSCODE) and create a directory and create a file with the extension .rs which indicates that the file is a rust file and enter the following code into the file

 ![image](https://user-images.githubusercontent.com/86939592/230705066-82e3181c-91ca-471c-a157-481f6ca34617.png)
 
 first to run this code we have to use the rust's built in compiler called <i>rustc</i> followed by the file name in our case `Hello_World.rs`.so enter the following command in your terminal.if you used gcc or g++ compiler to compile the c or c++ code it is same
 
 `rustc Hello_World.js`
 
The above command produces a `.exe` in the same directory which is binary file that can run directly on you system.

![image](https://user-images.githubusercontent.com/86939592/230705429-a3694cf5-61c3-4227-b5a8-52b1e471694b.png)

To run the binary file type in the following command in your terminal

`./Hello_World.rs`

and the output is

![image](https://user-images.githubusercontent.com/86939592/230705508-2742798c-3518-435b-b54b-fed54880d5ce.png)

wohooooo we successfully created and ran our first rust program which prints `Hello World!` to the screen

and now we go through each part in our code the first line is 

`fn main(){`


The fn keyword is rust's builtin keyword that is used to create a function to perform a task, in our case the function `main` is the first function that runs when we run the program and that curly brace at the end of the line indicates the function scope means the area that function is in charge and it is also paired with anothe opposite curly brace and the code between the 2 curly braces are under the function main.

and now the second line 

`     println!("Hello World!);`

the code in between the two curly brace is body of the function and every instruction in the function is have to end with an semicolon `;`.here `println!()` is a rust macro that means it is like a inbuilt function that is specifically to print the given argument to the standard output. we have given a string literal to it and it prints the string to the terminal.

the function ends when the execution reaches the opposite curly brace in the 3rd line and control goes back to the opearting system

### Bonus point
whenever you run rust program using rustc it produces the .exe file with the same name as program name.so to custimize output file name we can use the `-o` with the output file name

`rustc Hello_World.rs -o First`

then it produces the files like First.exe

## managing bigger project

compiling single file with rust compiler is works fine,but as the project grows bigger bigger with the n number of files compiling all of them individually and combining htem will be tedious task.That's where cargo comes in.

It is a rusts build system and it's package manager.it handles all the compiling and combining and downloading libraries things.libraryies in rust called as crates

cargo installs already when we installed the rust itself,so to confirm type in this command in terminal:`cargo --version` you will see a version number

### creating project with cargo

`cargo new test_cargo`

this command will create a direcoty like this

![image](https://user-images.githubusercontent.com/86939592/230785299-d34a703b-60f0-42a3-9d52-9d32944bdcc6.png)

go into the directory using the command `cd test_cargo`

here we have one directory and one file called `Cargo.toml` if you ever work with nodejs it is same sa the package.json which stores the metadata about the libraries and projects

after opening the Cargo.toml file it looks like this:

![image](https://user-images.githubusercontent.com/86939592/230785437-786f3841-496b-45b1-b0dd-00538bcf154b.png)

here in the first part `[package]` it means our project is package.package means group of codes that other programmers can use

after that we got all the basic info about our project

then there is a line `[dependencies]` it means that the all the required libraries for the projects is going to start from here (i.e..) the dependencies section starts here


and then we have a directory also called `src`, once we get into src folder we have a file named as `main.rs` and it contains a basic boilerplate code that we have already discussed in the previous section

![image](https://user-images.githubusercontent.com/86939592/230785638-ee4c41e3-6716-4644-b4bf-d05f974b429f.png)


and that's the tour to the cargo and to compile this project just type in this command : `cargo build` after running this command we got a target folder

![image](https://user-images.githubusercontent.com/86939592/230785754-4586a159-f5a2-424d-a68e-68cea9c2ca8f.png)

then in target folder we have a debug and inside debug we have `test_cargo.exe` which is our binary file,so to run this we can do like

`./target/debug/test_cargo.exe`

or else we can build and run the project simultaneously with one command: `cargo run` this will compile and run the project for you

![image](https://user-images.githubusercontent.com/86939592/230785929-f3b90e6d-9902-4266-8e9f-e904ddc0c41d.png)


and this is how you can handle complex projects with cargo



## Summary:

- rust is newly emerged language and it can do systems prorgramming and low level work with insane performance
- we learn how to write basic hello world prorgam and discussed about the syntax
- and also learned how to manage bigger projects with cargo

 ### Thank you 

This is the session for this chapter,see you on next part!!
