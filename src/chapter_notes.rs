// chapter 3: Common Programming Concepts
  // use mut in front of a variable in order to change it in the future
  // variables in general are immutable in Rust
  // in const you must always annotate the type. let and const are variable types
    // const declaration uses uppercase const VARIABLE_NAME
  // immuntable cant change but you can call the same variable and shadow existing variable
    // mut allows you to change variables shadowing allows you the reassing variable or shadow but keep them  immutable
    // shadowing lets you use the same variable without reassigning the name
    // Data Types: Rust is statically typed language so it must know types of all variables at compile time
      // Scalar: represents single values
        // integers, floating-point numbers, Booleans, and characters
        // signed(i) and unassigned(u) + memory space it takes; signed meaning whether it can be positive or negative
        // signed numbers are stored using two's complement representatins
          // signed variant can store numbers from -(2n-1) to 2(n-1) -1 n is the number of bits that variant uses
          // unasigned variables can store numbers from 0 to 2n -1
          // isize and usize depends on the kind of computer your program is running on 64 or 32
          // integer literals*
          // rust defaults to i32: generally the fastest on integers
          // you use isize or usize when indexing some sort of collection
      // Integerr Overflow: when you try to store values that are greater than what its primitive type can store
        // two compliment wrapping: the first overflow becomes 0 and the second becomes 1 and etc...
      // Floating-Point Types
        // dafault types of floats is f64 which is the same as f32 on modern CPUs but f64 is capable of more precision
            // f32 is single-precision and f64 is double precision
        // Booleans : true or false and they are 1 bit size
            // booleans are specified using bool let f: bool = false;
        // char type specified with single quotes and strings are specified with double quotes
        // Unicode Scalar Values from U+
      // Compound
        // tuple: general way of grouping together some number of other values with a variety of types into one compound type
          // fixed length, once declared they cannot grow or shrink in size
          // use pattern matching to extract values out of a tuple: and index it (x, y,z). destructuring
          // you can access a tuple using a period (.indexNumber)
        // array: unlike tuple, every element of an array must have the same type
          //  use array if values dont change instead use a vector, all elements in an array have the same type
          //
      // Functions
        // functions: fn wrtten in snake case
        // in function signature, you must declare the type of each parameter
      // Statement and Expressions
        // statements are instructions that perform some action and do not return a value; function declaration is a statement
        // expressions evaluate to a resulting value
        // you cant assign a variable to another statement in rust
        // expressions do not include a semi colon at the end, if you add a semi colon you turn the expression into a statement
        // return values: declare a function type using an (-> type)
      // Comments //
      // Control flows
        // if and loop are the main control flows of Rust
        // blocks of code in an if statementis often called arms
        // you must explicitly state a function as a boolean in Rust unlike other languages
        // if else else if () {}
        //rust only executes bthe block for the first true condition once it finds one, it doesnt check the rest
        // if is an expression so it can run next to a variable
        // numbers are an expression, when you compare integers be sure that you are comparing them to an expression.
        // variables must had a single type
        // lop is never ending unless stopped using break
        // while runds a loop as long as the code is true; loop runs until a condition is true
        // for loop for element in variable_Name.iter (.ter)
        // .rev reverse method and range (1..4) <- not including

  // Chapter 4: Ownership: very important to rust
        // accessing data in a stack is more organized than accessing data in a heap. stack is defined heap is not
        // Strings are stored on a heap
        // create strings using let s= String::from("hello"); :: used for namespacing
        // .push_str : method to push a string to an already defined variable
        // strings can be mutated but literals cannot. Stack vs heap
        // memory is returned once variable goes out of scope
        // once you reassign a variable the previous variable is invalid and the memory assigned to that varaible is then reassigned to the new variable
        // use .clone to copy a data / heap
        // integers are different from strings. assigned variables reassigned are still valid
        // strings lose ownership when they are re-assigned and integers just copy
        // when a variable goes out of scope the value will be cleaned up by drop unless ownership is transfered
        // References and Borrowing
          // using & allows one to reference a varaible but does not transfer ownership
          // references is the same as borrowing
          // attempting to modify a borrowed value does not work
          //references are mutable if you ad &mut to the declaration of the value
          // one mutable reference to a particular data per scope &mut
          // creating new scopes allows you to use mutuable references
          // you can have several references within a scope but only one mutuable referene within a scope
          // you can have an immutable and mutuable reference within a scope
          // Dangling References
        // Slices
          // .as_bytes() takes a string and turns it into an array
          // array.iter().enumerate() enumerate returns and index and the element
          // for (i = index, &item= references the elements within an array)
          // &s reference a word and  [0..range] defines how much you want from that word; &s[0..range]
          // &s ..= means we are including the last number or ... inclusive .. not invlusive
          // byte is an element in an array


  // Chapter 5: Structs to Structure Related Data
        // Struct is an aboject
          // define a struct as : struct User { key: value, key: value}
          // arrows defines what a function returns
          // use feild init shorthand syntax to rewrite structs that are created in another function
          // when creating an object(struct), you can re-use other struct in the creation a new struct and just changr what you want to change
          // tuple structs have the same meaning as stucts but they have different naming convention
          // unit-like-structs: structs that don't have any fields
          // storing references in a stucts without lifetime wont work without lifetime
          // structs dont have a provided implementation of display unless you use {:?}
          // #[derive(Debug)] : used in debugging
          // or use {:#?} in bigger outputs
          // structs may be instantiated let origin = Point { x: 0. y:0}
          // structs are accessed with dot notation
          // structs are private by default but can be made public using (pub)
          // modules are name spaced with structs that are defined within
          // updating structs you, reassign the values wanted to change but what you dont change you can use ..x to copy
          // struct can be instantiated with out a value
        // Methods syntax
          // impl used define a method within an object
            // can create method for structs and enums;
          // Self: determines what kind of ownership in a method
            // &self: the method borrows the value: commonly used
            // &mut self: method borrows an modifies struct
            // self : takes ownership
          // rare that a method takes ownership of an object
          // . to call methos on a object: rust automatically adds in &, &mut, or * on object when you call a method
          // rust makes borrowing implicit in methods
          // Create a struct and then define within a fuction of the struct different method its takes
          // using impl(implementation) : allows definining functions within an object without using it as a method

  // Chapter 6: Enums and Pattern Matching**
        // enum is a way of expressing some data that may be one of several things (datatypes)
        // structs contain one data type: struct contains all asians and an enum contains black, white, asians. Objects with similarity vs Objects unique to themselves
        // variants of enum are namespaced under its identifier ::
        // enums allows one to store different datatypes
        // you can pass any type of parameters within an enum; enum variable_name { variant_Name()}
        // enums are several structs within a function
        // enums just like structs have methods
        // Structs & enums are stored inline by default, so they may not be recursive
          // inline meaning elements are stored a called on every where they are called so recursive calls must be specified or you have an infinte size at compile time
          // box is a term for allocating data on a heap
          //Box<variable_name> is a heap pointer with exactly one owner
          //
        // option
         // rust doesnt have null value
         // use of None Some(T)
         // option<T> means a value may or may not exit: wild card
         // helpful for objects that can be created with missing values. option<T> doesnt give you null but rather allows you to work without input
         // every option must be accompanied with a Some(i) and a None incase some does not exist or a placeholder
        // Match
          // ref used in match statements to reference a variable
          // if-let
          // while-let : keeps running until condition fails to match
          // binding() passing enums that are binding to other enums or objects
          // _ Placeholder
            // _ placeholder ignores all tools before
        // Concise Control Flow with if let
          // if let is a more concise match statement
            // if let statement = expression {}
            // sugar syntax for match

  // Chapter 7: Modules
        // mod declares a new module
        // cargo new project_name --lib
        // cargo build used to compile crates : equal to bundle in rails
        // cargo run == rake
        // module are good for grouping
        // *difference between library crate and binary crate*
        // modules call modules that are already defined with functions
        // use allows you to extract values from a mod with out explicitly naming each value
        // each module has its own folder
          // module -> module.rs (all functions go in there) -> submoduel(name of the module as a file -> module.rs)
          // lib.rs calls the main modules
            // mod A; and mod B;
              // mod A does not have a sub file: the file will just be A.rs
              // mod B has sub files so the foler will have a mod.rs file with a function and call mod C a file within mod B.rs
              // C.rs will contain the functions that are attached to it: inheritance with namespace
        // Public vs Private
          // default state in all rust code is private
          // use pub to make modules public
            // use allows a shortened version of namespacing
            // use TrafficLight::{Red, Yellow}; enums
            // * to bring all into scope * global scope
            // super used to move up one module in a hierarchy
            //

  // Chapter 8: Common Collections
    // collections are stored on a heap
    // Vectors: Vectors can only store values of the same type
      // usefule to for list items: text file or prices of items in a shopping cart
      // heap is good for when you have data that has many occurence then the pointer can take you to the group that sopecific to a data
      // v: datatype Vec<i32> = Vec::new()
      // updating vec: v.push(value)
      // when you leave the scope of vectors, they are droped
      // you can use either index number or .get method to access values in an index
      //  use index &v[] method if want the program to crash if user access goes out of range but if its common that the vector usually goes out of range, you can use .get
      // since vectors require memory; retrieving a value and updating will not work at once
      // use for look to itterate through a vector
      // use * operate to get the value in a vector in order to change
      // in order to use a vector with different data type; you store enum in the vectore
      // vec! used to instantiate a new vector
    // Strings
      // .to_string used to create a string
      // String::from(literal) : used to create strings from literals
      //push_str : used to update a string, add string to a string
      // use .push to push characters to a string
      // format!({}{}{}, match) used for more complex concatnation
      // strings are saved on a heap as a UTF8 code so accessing each char is impossible because they arent saved by the index number
      // use slicing with a range to define what you want to extract from a data set
      // first turn the string into array using .char .bytes: return raw bytes
    // Hash Maps
      //

  // Chapter 10: Generic Types, Traits, and Lifetimes
      // <T> to instantiate a genric; you pass that generic value to your function or variable type with what data type that coincide with the function or variable













