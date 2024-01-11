
# Arc Interpreter 🪄

Welcome to the Arc Interpreter project! 🚀 This Rust-based interpreter is a learning project aimed at understanding how interpreters and compilers work, while concurrently gaining proficiency in the Rust programming language.

# Overview 📸

This project follows the principles outlined in the book "Crafting Interpreters" by Robert Nystrom, serving as a practical guide to delve into language design and implementation.

## Learning Objectives 🏁

- **Understanding Interpreters and Compilers:** Explore the fundamentals of interpreter and compiler design through practical implementation.
- **Rust Language Proficiency:** Gain hands-on experience with Rust and its unique features.

# What I Learned Along this Journey? 🐴
During the Arc interpreter development journey, I acquired essential lessons:
- [x] **Interpreter Design Essentials:**
	-   Lexical Analysis and Code Tokenization
	-   Recursive Descent Parsing Techniques
	-   Implementation of Tree-Walk Interpreters
	-   Language Design Principles
	
 - [x] **Rust Syntax & Fundamentals**
	 - Basic Rust Syntax , borrowing & ownership.
	 - Smart pointers using `Rc` & `RefCell`  
	
- [x] **Code Proficiency:**
	-  Explored unit tests and how they ensure code reliability.
	-  Applied design patterns to create well-structured and efficient code.

# Getting Started 🚦

### **1. Clone the Repository**

Begin by cloning the Arc repository to your local machine. Open a terminal and execute the following command:
```
git clone https://github.com/aym-n/arc.git
``` 

### **2. Navigate to Arc Directory**
Move into the Arc directory using the `cd` command:
```
cd arc
``` 
### **3. Build and Run Arc**
To run a Arc script, use the following command, replacing `<filename.arc>` with the path to your Arc script:
```
cargo run <filename.arc>
``` 
If you want to run the Arc REPL (Read-Eval-Print Loop), simply execute:
```
cargo run
``` 
### **4. Explore the REPL**
The REPL provides an interactive environment for experimenting with Arc. It includes some useful commands:

-   To exit the REPL, type `exit` and press Enter.
-   Clear the screen using the `clear` command.
-   Print the current environment conditions by typing `@`.
# Docs 📚️
This quick start provides a brief overview of Arc, a dynamically-typed scripting language. Dive into expressions, statements, and the fundamentals of object-oriented programming with Arc's straightforward syntax. Start coding with Arc and experience the elegance and simplicity of its syntax.

## Dynamic Typing
Arc adopts a dynamic typing system where variables can store values of any type. A single variable has the flexibility to store values of different types at different times. If an operation is performed on values of incompatible types, such as dividing a number by a string, errors are detected and reported at runtime.

## Data Types
The fundamental building blocks are Arc's built-in data types, each representing the atoms that compose all matter. Let's explore the few essential data types in Arc:

### Booleans
```
true;   ~ Not false.
false;  ~ Not *not* false.
```
### Numbers
Arc features a single numeric type: double-precision floating point. This choice allows representation of both decimals and a wide range of integers, simplifying the language.
```
1234;   ~ An integer.
12.34;  ~ A decimal number.
```
### Strings
Strings in Arc are enclosed in double quotes and support various literals.
```
`"I am a string";
"";      // The empty string.
"123";   // This is a string, not a number.
```
### Nil
Representing "no value," nil is the built-in value that often shows up uninvited. While it is called "null" in many languages, Arc spells it as nil.
```
nil; ~ nothing here
```
These fundamental data types provide the basis for constructing more complex structures and operations within Arc
## Expressions
If built-in data types and their literals are atoms, expressions can be considered the molecules. Let's delve into the familiar and essential expressions within Arc.
### Arithmetic
Arc supports basic arithmetic operators, akin to those found in C and other languages:
```
add + me;
subtract - me;
multiply * me;
divide / me;
```
These binary operators operate on numeric operands. The `-` operator, besides being infix, can also serve as a prefix for negating a number:
```
-negateMe; 
```
Notably, the `+` operator can concatenate two strings.
```
"Hello" + "World";
```
### Comparison and Equality
Leveraging comparison operators, Arc enables the evaluation of Boolean results. Numeric comparisons and equality checks are supported:
```
less < than;
lessThan <= orEqual;
greater > than;
greaterThan >= orEqual; 
```
Additionally, Arc allows testing values of different types for equality or inequality:
```
1 == 2;         ~ false.
"cat" != "dog"; ~ true.
314 == "pi";    ~ false.
123 == "123";   ~ false.`
```

### Logical Operators

Logical operators in Arc include the prefix `!` for negation, as well as `and` and `or` for conjunction and disjunction:
```!true;  // false.
!false; // true.

true and false; // false.
true and true;  // true.

false or false; // false.
true or false;  // true.`
```
These operators also serve as control flow structures, with short-circuiting behavior.

### Comments
Comments are essential for documenting your code and providing context. Arc supports comments by using the `~` symbol
```
~ Comments here

~ Hello
~ World!,
~ Get started with arc
```

### Precedence and Grouping

All operators maintain the expected precedence and associativity from C. To alter precedence, grouping with parentheses `()` is available:
```
var average = (min + max) / 2;`
```
The remaining typical operator functionalities like bitwise, shift, modulo, or conditional operators are omitted in this concise representation. 

## Statements

Now we're at statements. While an expression's primary role is to produce a value, a statement's purpose is to generate an effect. Since statements don't evaluate to a value, they need to alter the world in some way—usually by modifying state, reading input, or producing output.
```
print "Hello, world!";
``` 

A print statement evaluates a single expression and displays the result to the user. 
If you need to group a series of statements where a single one is expected, you can enclose them in a block:
```
{
  print "One statement.";
  print "Two statements.";
}
```
Blocks also influence scoping, which we'll explore in the next section...
## Variables
Variable declarations utilize the `var` statement. If you omit the initializer, the variable's value defaults to `nil`.
```
var imAVariable = "here is my value";
var iAmNil;
```
Once declared, you can naturally access and assign values to a variable using its name.
```
var breakfast = "bagels";
print breakfast; // "bagels".
breakfast = "beignets";
print breakfast; // "beignets".
```
It's worth noting that variable scope follows familiar patterns, akin to expectations from languages like C or Java.
## Control Flow
Effective programming involves the ability to skip or repeat code as needed. In addition to the logical operators covered earlier, Arc incorporates three statements borrowed from C to facilitate control flow.

An `if` statement executes one of two statements based on a specified condition:
```
if (condition) {
  print "yes";
} else {
  print "no";
}
```
A `while` loop repeatedly executes the body as long as the condition expression evaluates to true:
```
var a = 1;
while (a < 10) {
  print a;
  a = a + 1;
}
```
Although Arc excludes `do while` loops due to their relative rarity
Lastly, Arc supports `for` loops:
```
for (var a = 1; a < 10; a = a + 1) {
  print a;
}
```

This `for` loop achieves the same result as the previous `while` loop. Arc maintains simplicity by eschewing more advanced loop constructs found in some modern languages, keeping it fundamental.

## Functions
In Arc, a function call expression resembles its appearance in C:
```
makeBreakfast(bacon, eggs, toast);` 
```
You can also call a function without passing any arguments:

```
makeBreakfast();` 
```
Unlike certain languages like Ruby, the parentheses are mandatory in this case. Omitting them doesn't invoke the function but rather refers to it.
To define your own functions in Arc, use the `fn` keyword:
```
fn printSum(a, b) {
  print a + b;
}
```
Let's clarify some terminology:
-   An **argument** is an actual value passed to a function during a call.
-   A **parameter** is a variable within the function that holds the value of the argument.

Functions in Arc don't require a separate declaration and definition, as the dynamically-typed nature simplifies this distinction. The body of a function is always a block, where you can use the `return` statement to return a value:
```
fn returnSum(a, b) {
  return a + b;
}
``` 

If a function's block reaches its end without encountering a `return` statement, it implicitly returns `nil`.

## Closures

Arc treats functions as first-class citizens, allowing you to reference, store, and pass them around. Local functions can be declared inside another function:
```
fn outerFunction() {
  fn localFunction() {
    print "I'm local!";
  }

  localFunction();
}
``` 
Combining local functions, first-class functions, and block scope introduces closures:
```
fn returnFunction() {
  var outside = "outside";

  fn inner() {
    print outside;
  }

  return inner;
}

var fn = returnFunction();
fn();
``` 
Here, `inner()` accesses a local variable declared outside its body. This behavior is acceptable and known as closures, where functions retain references to surrounding variables even after the outer function has returned.
## Classes
In Arc, we introduce a simplified and flexible approach to object-oriented programming, focusing on classes and instances. Let's explore the key concepts and features.

### **Class-Based and Prototype-Based Systems**

Arc incorporates class-based object orientation, which is more prevalent in languages like C++, Java, and C#. However, it acknowledges the existence of prototype-based systems, exemplified by JavaScript. In practice, the line between these two approaches can blur.

### **Classes in Arc**
Classes in Arc consist of methods, declared within the class body. Unlike functions, no `fn` keyword is required.
```
class Breakfast {
  cook() {
    print "Eggs a-fryin'!";
  }

  serve(who) {
    print "Enjoy your breakfast, " + who + ".";
  }
}
``` 

Upon class declaration, Arc creates a class object, stored in a variable named after the class. Classes in Arc are first-class entities, allowing them to be stored in variables and passed as arguments to functions.
```
var someVariable = Breakfast;
someFunction(Breakfast);
```

### **Instantiation and Initialization**

To create instances, Arc uses the class itself as a factory function. Invoking a class produces a new instance;
```
var breakfast = Breakfast();
print breakfast; // "Breakfast instance".
```
Classes in Arc can have fields to encapsulate state. You can freely add properties to objects:
```
breakfast.meat = "sausage";
breakfast.bread = "sourdough";
```
To access fields or methods within a class method, use `this`:
```
class Breakfast {
  serve(who) {
    print "Enjoy your " + this.meat + " and " +
        this.bread + ", " + who + ".";
  }
  // ...
}
```
### **Instantiation and Initialization**
Encapsulating behavior and state in object-oriented programming involves defining fields. Arc allows dynamically adding properties to objects.
```
breakfast.meat = "sausage";
breakfast.bread = "sourdough";
```
To ensure objects are in a valid state upon creation, define an initializer. If a class has an `init()` method, it's automatically called when the object is constructed:
```
`class Breakfast {
  init(meat, bread) {
    this.meat = meat;
    this.bread = bread;
  }
  // ...
}
var baconAndToast = Breakfast("bacon", "toast");
baconAndToast.serve("Dear Reader");
// "Enjoy your bacon and toast, Dear Reader."` 
```
### **Inheritance**
Arc supports single inheritance. Use the less-than (`<`) operator to specify a superclass when declaring a class.
```
class Brunch < Breakfast {
  drink() {
    print "How about a Bloody Mary?";
  }
}
```

In Arc, every method in a superclass is available to its subclasses:
```
`var benedict = Brunch("ham", "English muffin");
benedict.serve("Noble Reader");` 
```
To inherit constructors, use the `super` keyword:
```
class Brunch < Breakfast {
  init(meat, bread, drink) {
    super.init(meat, bread);
    this.drink = drink;
  }
}
```

These fundamental features form the basis of object-oriented programming in Arc, providing a balance between simplicity and functionality.

## Resources 🛜

- [Crafting Interpreters Book](https://craftinginterpreters.com/): The essential resource guiding this learning journey.
- [Rust Programming Language](https://www.rust-lang.org/): Learn more about Rust.

---

**Happy Learning! :)**



