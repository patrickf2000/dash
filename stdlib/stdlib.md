## Lila Standard Library

Contains the functions that should be defined in a Lila runtime system. This is also used to indicated what is currently implemented and what we still need.

The Lila standard library is not essential, but highly recommended. Please see the corresponding documentation for the core library.

### text_io

* printf -> Works similar to C printf   
* printInt -> Output an integer   
* printHex -> Output an integer as a hex value   
* printFloat -> Output a floating point value   
* printDouble -> Output a double value
* readLn -> Read a string from standard input   
* readInt -> Read an int from standard input   

### file_io

* getByte -> Read a byte from a file   
* getLine -> Read a line of text from a file   
* writeByte -> Write a byte value to a file   
* writeLine -> Write a line of text to a file   

### io

Note that most of these are wrappers around Linux system calls

* open -> Open a file for reading   
* create -> Create a new file   
* read -> Read from a file   
* write -> Write to a file   
* lseek -> Move the position in a file   
* close -> Close a file   

### mem

* resize -> Resize an array   

### string

* str2int -> Convert a string to an integer   
* int2str -> Convert an integer to a string   
* strcat -> Join two strings   
* str_append -> Append a character to a string   

### math

* pow -> Raise a number to a power   