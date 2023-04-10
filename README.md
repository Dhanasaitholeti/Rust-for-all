# <p align="center">Rust-for-all</p>

hey guys this is day2 and today we are going to learn about rust via a project and the project is a game

we are going to programming a guessing game to learn things 

so buckel up and get ready to learn about the stuff

so..first we are going to create a directory for the game by the below command:

`cargo new guessing game`

![image](https://user-images.githubusercontent.com/86939592/230872010-644a38db-f842-4476-acac-ddb6e755776e.png)

It will create a folder with a toml file and we are ready to go

as you already know from the previous chapter that we got a project with `src` directory and a `.toml` which is works like a config file for rust project and also we got a `main.rs` file which contains default boilerplate code init which prints `Hello World!` to screen

### processing a guess

The first form of the processing task is to ask user for a input and validate that the given input is in correct form.

![image](https://user-images.githubusercontent.com/86939592/230873503-98f03298-b620-4629-bab7-f568f78ab5a7.png)

There is alot of information in the above program let's go over it line by line.

The first line is `use std::io`,it is a rust's buitin library that is necessary to take user input and then printit into the screen.generally rust doesn't load the all types into the scope to use the external types we need to import then explicitly using the `use` keyword

Then on the second line we declared a function using the `fn` keyword which is used to define functions aswe did in the previous lesson and empty paranethesis `()` indicate that this function doesn't have any parameters.
 
 on 5&6 line we got a macro that we already discussed in previous chapter.
 
#### Storing values with variables

on line 10 we are storing a string value in guess variable,here the program is getting too interesting 

`let foo = bar;`

the above code declares a a variable `foo` and assigning the value of the bar to it

you have already observed there is no `mut` keyword in the below example,This is because,In Rust the variables are *immutable* by deafault that means we cannot change the value of the variable while running the program

back to the game we now know that it creates a mutable varibale named guess.on the other side of hte `=` we got `String::new`

:: in the ::new is indicates that it is a associated function of the type String,It is works same as the static methods in some languages,they work on the type rather than a instance of the type.Here it indicates that the ::new is a associated function of the String type.

To summarize the above line we have created a mutable variable with a empty string value.ohoooo.....!!!


On the line 12 we got a line like this:

`io::stdin().read_line(&mut guess).expect("failed to read the line)`

Recall that on the first line of the we have included the capability of input/output from standard library.so now we are going to use `stdin()` method from the `io` module,if we hadn't listed the `std::io` at the first line of the program we have write the above line as `std::io::stdin()`

and next part of the code is `.read_line()`.The work of the read_line methode is that takes whatever the user typed into the standard input and assign the value to the whatever the argument passed, see that we have also passed `&mut guess`.

The `&` tells that it is refernece of the mutable variable guess so that it can replace the guess variable with whatever the user typed into the standard input.

and the further part is `.expect()` method which is used as the  error handling methode which catcher the errors,we can do advanced error handling with different methods and it is a basic one

In the last line of the function body we print the whatever the guess variable have to the screen or standard output.


so enter the given code in your tect editor and try to run using the command `cargo run`.The output will be like below screenshot

![image](https://user-images.githubusercontent.com/86939592/230879134-9e5454c1-6d3a-419e-bf41-4ff8a87b39ab.png)




