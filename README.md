# CSCI3055U Final Project: Rust Programming Language

Adam Bozzo
adam.bozzo1@uoit.net

## About the language

*History*

The rust programming language started out of a project in 2006 by Graydoan Hoare. Rust was officially announced in 2010 and the compiler was up and working by 2011. Its first stable release was in 2015 and since then has been constantly improved upon.
      
*Interesting Features*

Some interesting features about rust are:
- you do not need to declare your type of variable, similarly to python this is inferred by the compiler
- allows you to have the optionality of deciding whether variables are mutable or not
- A feature on rust called ownership enables rust to have very safe memory guarentees without a garbage collector.

## About the syntax

*Variables*

In rust you use the let keyword to declare variables. When creating variable names in rust, for the basic variables they must always be snake name case. This means that they must be formated as "name_second", you cannot have a name as "nameSecond" for example. The variables that you declare are by default immutable, this means that you cannot assign them new values. For example, in the following snippet of code y will not be reassigned to 5 since y is immutable.
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

*Functions*

Every rust program starts at the main function, the fn main(){}. You can also create other functions and call them from the main function. You can also set paramaters as well as return values from them. An example of this is:
```
fn main(){
     println!("Main function start!");
     let x = second_function(10);
     println!("x");
}
fn second_function(y: i32)-> i32{
      y + 40
}
```
Note that the -> i32 indicates that it is returning an integer, if you wanted to return nothing you would leave that empty. Also note that on the line y+40 you do not use a semi-colon in order to indicate it is returning. When you are indicating the paramter receiving you use name: datatype, in order to declare the variable to the appropriate data type.

*Conditionals*

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

*Loops*

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
*Compile and Run Tools*

In rust there are two different ways you can compile and run your program. The first way would be through using the compiler rustc to compile your program, once compiled you can run your program using ./nameofcompiledprogram. You can get rustc by using the command:
"curl https://sh.rustup.rs -sSf | sh" in your linux terminal, or you can visit the website https://www.rust-lang.org/tools/install for more information on how to install it when using different operating systems. Rust is very similar to C++, in which you compile your program and then run it unlike other languages where it is all done at once. Additionally, you can use Cargo to compile and run programs. Cargo is a Rust build system and package manager, cargo is specifically very good at managing projects as well as adding dependencies and other libraries. It is basically required to use cargo when doing any sort of large scale project with mutliple dependencies. Cargo will compile the program and run during one command as opposed to rustc where you must first compile and then run. In order to create a cargo project you need to type cargo new "project_name" and in order to compile and run the program you type cargo run in your terminal/shell. There are also some different IDE's that you can use to compile and run your code, an example of some of these would be IDEA and Emacs.

## About the standard library

*Standard Library*

Rust has a very large standard library that provides multiple different data structures that can be used. Some of these data structures that I will be going over are hash maps, vectors and queues.

The first collection I will be talking about are hash maps, hashmaps are a very useful standard library that allow you to store values based on their keys. Different methods for this library are:
"name".insert(key, value); This allows you to insert a key and value into the hash map
"name".contains_key("key"); this returns a boolean that checks to see if the key exists in the map
"name".remove("key"); this removes a key from the hashmap

An example of a hashmap in rust and iterating over the map is:
```

use std::collections::HashMap;
let mut names_grades = HashMap::new();

names_grades.insert(
	"John".to_string(),
	54,
);
names_grades.insert(
	"Adam".to_string(),
	74,
);
names_grades.insert(
	"Jessica".to_string(),
	92,
);
for (names, grades) in &names_grades {
    println!("{}: \"{}\"", names, grades);
}
```

The second collection I will talk about is the vector collection. The strength with vectors is that it is a dynamically sized array, meaning you can contiuously change the size of it without having to reinstansiate the array. Some different methods that you can use with a vector are:
"name".push(value); this pushes a specified value onto the vector
"name".pop(); this removes the top most element of the vector and returns it
"name".len(); this returns the size of the vector
"name".extend(); will extend the vector with values specified

An example of a vector is:
```
let mut vec = Vec::new();
vect.push(40);
vect.push(20);
vect.push(10);
vect[1] = 15;
for index in &vect {
    println!("{}", index);
}
```

An alternative way to declare vectors is using the vec! keyword. An example of this is:
```
let vect = vec![10,20,30,40,50];
```

The third standard library collection I will be going over is the VecDequeue collection. This collection basically acts as a list where it is very volatile and can be used with all 4 variants (First in First out, First in Last out, Last in First out and Last in Last out). Queues iterate front to back. Some different methods you can use with a VecDequeue are:
"name".push_back(value); this adds a value to the back of the queue
"name".pop_front(); this removes the front of the queue and returns it
"name".pop_back(); removes element from the back of the queue and returns it 
"name".push_front(value); adds element to the front of the queue
"name".swap(index 1, index 2); this swaps two elements in the queue

An example of a queue this in rust is:
```
use std::collections::VecDeque;
let mut queue = VecDeque::new();
queue.push_back(3);
queue.push_back(4);
queue.push_back(5);
queue.pop_front();
```

## About open source library

*Open Source Contributions*

There are many different open source contributions for rust, the one that I will be talking about is the Actix web open source library. This library can be found at https://github.com/actix/actix-web. This open source library is a fast web framework designed for rust. Its main purpose is to provide a way to create a web framework from rust efficiently and fast. It is very useful when developing servers. Some examples of the different things it can provide are: client/server WebSockets support, streaming and pipelining, confiurable request routing, has a http client, as well as many other things. It is build ontop of the other open source library called Actix actor framework. In order to use it you must included it in your dependencies under your cargo project.

# Analysis of the language

*Analysis of Language*
Rust is not a functional programming language, it is a procedural. Despite the fact that it is not primarily a functional programming language it is possible to code programs using a functional style in rust, this gives the language much versatility. Rust does in fact support meta programming and has different pattern matching tools. It has a very simple to use and powerful macro system that enables meta programming. Macros in rust basically look like functions but they end with a !. Macros are expanded into abstract synax trees which allows them to not give unexpected bugs. An example of a declaration for a macro tool would just simply be declaring it with the keyword macro_rules!, example of this is:

```
macro_rules! test{ 
	() =>(
		println!("testing!!");
	)
}
```
Rust also supports closure which allows us to wrap up free variables for easier reuse and clarity. When the free variables are used in a closed scope they become closed over meaning they are free to be used later on. An example of rust using closure is:
```
let adding_stuff = |y: i32| y + 5;
```
The variable y in this example is specificly stuck in the scope of adding_stuff and is a free variable once this line is finished executing.

Rust supports lexical scoping much like more of the major programming languages such as Java and C++, meaning that based on where the variable is declared it will be mutable and useable at any lower level scopes. It is not like dynamic scoping where it is more based on the specific scope you are currently working in. Rust also supports dynamic types through the use of something called trait objects and static/dynamic dispatches. Rust primarily focuses on favoring static dispatch but it also can do dynamic dispatching through trait objects. This allows us to change the type of the object we are working with. Some disadvantages of the rust programming language is that it is very explicit which sometimes can create very convoluted code and be jarring for beginners. Almost everything in rust is done very explicitly and in your face which allows a greater degree of control as a programmer but can sometimes be too much. Another disadvantage of the rust programming language is that it is very lack luster in comparison to other langauges in terms of supporting graphic user interfaces. Some advantages of rust is that rust is a very safe language to use, it is also very fast in comparison to a language such as java. It also has very good pattern matching abilities and overall is a very versatile language that gives you many different options on what you want to do.



