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

## About the tools

> _Describe the compiler or interpreter needed_.

## About the standard library

> _Give some examples of the functions and data structures
> offered by the standard library_.

## About open source library

> _Describe at least one contribution by the open source
community written in the language._

# Analysis of the language

> _Organize your report according to the project description
document_.


