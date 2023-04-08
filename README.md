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
