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
          // &s ..= means we are including the last number
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
          // updating structs you, reassing the values
        // Methods syntax
          // impl used define a method within an object
          // rare that a method takes ownership of an object
          // . to call methos on a object: rust automatically adds in &, &mut, or * on object when you call a method
          // rust makes borrowing implicit in methods
          // Create a struct and then define within a fuction of the struct different method its takes
          // using impl(implementation) : allows definining functions within an object without using it as a method

  // Chapter 6: Enums and Pattern Matching**
        // variants of enum are namespaced under its identifier ::
        // enums allows one to store 4 different
        // you can pass any type of parameters within an enum; enum variable_name { variant_Name()}
        // enums are several structs within a function
        // enums just like structs have methods
        // option
         // rust doesnt have null value
         //










