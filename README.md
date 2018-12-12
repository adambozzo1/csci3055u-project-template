# CSCI3055U Final Project: Rust Programming Language

Adam Bozzo
adam.bozzo1@uoit.net

## About the language

> _Describe the language_
>
> - History
      The rust programming language started out of a project in 2006 by Graydoan Hoare. Rust was officially announced in 2010 and the compiler was up and working by 2011. Its first stable release was in 2015 and since then has been constantly improved upon.
      
> - Some interesting features

## About the syntax

> _give some code snippet of the language_

*Variables*

In rust you use the let keyword to declare variables. The variables that you declare are by default immutable, this means that you cannot assign them new values. For example, in the following snippet of code y will not be reassigned to 5 since y is immutable.
```
let y = 10;
y = 5;
```
In order for your variables to be mutable you must use the mut keyword when declaring them. In this example x will be reassigned.
```
let mut y = 10;
y = 5;
```
Additionally, even if the variable is mutable, it cannot change types. So if you have a string type you cannot reassign it to an int.
The code below will not work.
```
let mut y = "Hello";
y = 10;

```
In rust you can also declare constant variables that are not able to be altered, you use the const keyword to specify them and they would be used for variables that you want to always remain the same like gravity acceleration.

```
const GRAVITY = 9.81;
```

Rust also has arrays that can store multiple data into one structure. An example of an erray is and how to print its values based on index is:
```
let array = [20,30,40,13,102,43];
println!("Array index 0: {}", array[0]);
println!("Array index 1: {}", array[1]);
```

If statements in rust are very useful for when you need a conditional statement. An example of an if statement and an if else statement syntax is:
```
let y = 4;
if y == 3{
  println!("y is 3");
}else if y = 5{
  println!("y is 5");
}else{
 println!("y is not 3 or 5");
}
```
An if let statement will evaluate the condition to see whether it is true or false and then change the variable in the let statement based on these conditions. An example of this is:
```
let y = 4;
let number = if y == 4 {
      5
      } else{
          6
      };
```

Loops in Rust:

In rust there are many different types of loops, the first one we are going to look at is the "loop" loop. This loop will repeatededly run forever without conditions. A way to break out of this loop would be to use the break keyword with conditions. You can also use loops to return a value when it breaks. An example of this loop is:

```
let mut i = 0;
let result = loop{
    i += 1;
     if i == 5 {
        break i;
     }
};
```

Another loop that is frequently used is the "while loop". This loop has a conditional evaluation in it and when that condition is broken the loop stops running. An example of this is:
```
let mut i = 0;
while i != 10{
   println!("i is not 10!");
   i += 1;
}
```

Lastly there is a a "for loop". This loop is very useful when going over specific ranges or collections. An example of a for loop going through an array is below:
```
let array = [2,3,4,5,10];
for element in array.iter(){
   println! ("value of array is: {}", element);
}
```


## About the tools

In rust there are two different ways you can compile and run your program. The first way would be through using the compiler rustc to compile your program, once compiled you can run your program using ./nameofcompiledprogram. You can get rustc by using the command:
"curl https://sh.rustup.rs -sSf | sh" in your linux terminal, or you can visit the website https://www.rust-lang.org/tools/install for more information on how to install it. Rust is very similar to C++, in which you compile your program and then run it unlike other languages where it is all done at once. Additionally, you can use Cargo to compile and run programs. Cargo is a Rust build system and package manager, cargo is specifically very good at managing projects as well as adding dependencies and other libraries. Cargo will compile the program and run during one command as opposed to rustc where you must first compile and then run. In order to create a cargo project you need to type cargo new "project_name" and in order to compile and run the program you type cargo run in your terminal/shell. There are also some different IDE's that you can use to compile and run your code, an example of some of these would be IDEA and Emacs.

## About the standard library

> _Give some examples of the functions and data structures
> offered by the standard library_.

## About open source library

> _Describe at least one contribution by the open source
community written in the language._

# Analysis of the language

> _Organize your report according to the project description
document_.


