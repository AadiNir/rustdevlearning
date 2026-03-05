This is a repo centric to my rust learning

=> cannot change the data type (compilation error)
=> rust is a systems language => lot of access to resources in the machine, will get access to RAM memory like a low level language, and also incredibly fast 


Memory Managment in rust:

=> Like the famous language JS, C++ allocates and deallocate memory on the RAM
=> usually other language will use garbage collector will clean up the RAM which is usually be in language like java, JS contrary we cant also access specific memory managment
=> Manual way to access memory allocation, which is usually done in C like malloc and alloc, can lead to dangling pointers/memory issue, the most famous language is C  
how rust does
=> Rust will give all memory managment which is same as C but they have certain bounds which will help in better memory managment which make sure the memory managment more safer, garbage collector makes a language slow, which actually makes rust fast

1) Mutability
2) Heap and memory
3) ownership models
4) Borrowing and references 
5) lifetimes

=> By default any variables in Rust is immutable let a:i32 is immutable you should explicitly mention if you want to change it let mut a: i32 something like this
=> immutable make sure no race condition during multi thread access, if making mutable everything should be synchronized so that even during multi threading it can accees the correct data


compound types

#fixed size array with data type
let tup: (%str,i32) = ("hey how r u",1000);

//Ownership
rust give u access to heaps and stack memory, stack is a fixed size memory, heap is a dynamic memory what will rust does it stores the variable in heap if it think is dynamic but then it will create the variable identifier in stack and points at heap considering accessing values through stack is a very easy
The Ownership Model in Rust is a unique approach to managing memory. It offers the benefits of manual memory management, such as fine-grain control over memory, faster runtime performance, and a smaller program size, while also guaranteeing memory safety. This is achieved through compile-time checks that prevent common memory-related bugs, although it may lead to a steeper learning curve due to strict rules and the "borrow checker."

Understanding Stack & Heap is crucial for Rust developers. The stack is a fixed-size memory region used for storing fixed-size variables and function call frames, with data living as long as its stack frame. The heap, on the other hand, is for dynamic-sized data that can grow or shrink at runtime, and its lifetime is controlled by the program. Pushing data to the stack is faster than allocating on the heap, and accessing stack data is quicker as it doesn't require following pointers.

The video outlines three Ownership Rules that are fundamental to Rust:

Each value in Rust has a variable designated as its owner.
There can only be one owner for a value at any given time.
When an owner goes out of scope, the associated value is automatically dropped (deallocated).
Variable Scope dictates the validity of a variable, meaning a variable is only valid from its declaration until the end of its enclosing scope. Rust automatically handles Memory & Allocation for dynamic data types like String. When a String is declared, Rust allocates memory on the heap, and when its owner goes out of scope, Rust automatically deallocates that memory. Simple types like integers, booleans, and characters that implement the "Copy" trait are copied rather than moved when assigned, unlike dynamic types which are "moved" by default, invalidating the original variable.

Ownership & Functions explains how ownership is transferred when values are passed as arguments or returned from functions. Passing a value into a function moves its ownership to the function's parameter, making the original variable invalid after the call (unless it's a "Copy" type). Similarly, returning a value from a function moves ownership to the variable receiving the return value. This mechanism ensures that memory is always managed by a clear owner.

References & Borrowing offer a way to use data without taking ownership. By passing a reference (using &), a function can "borrow" the value without invalidating the original variable. References are immutable by default, preventing modification of the borrowed value. To allow modification, a "mutable reference" (&mut) can be used. A key restriction is that there can only be one mutable reference to a piece of data in a given scope, or any number of immutable references, but not a mix of both. This rule is crucial for preventing data races at compile time. The video also touches upon "dangling references," where a reference points to invalid memory, which Rust proactively prevents at compile time.

Finally, The Slice Type allows you to reference a contiguous sequence of elements within a collection (like a part of a String or an array) without taking ownership of the entire collection. Slices are useful for safely working with portions of data, as they are tied to the original data's lifetime. They address issues where returning an index might become invalid if the underlying data changes. String literals are also discussed as a type of string slice.

