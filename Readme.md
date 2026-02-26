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