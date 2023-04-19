# What is it?

That's my Rust software security assignment during my major at University of Twente 🇳🇱. I was supposed to write, test and secure by all means my implementation of **Binary Search Tree** and its `CLI` interface. 

You know, I had some experience of doing basic `BST` during my bachelor and further career, but this time it was rust and software security specific, so it was interesting to grasp Rust approach to drastically limit the range of possible programs with its type system, thus eliminating malicious ones. 

# How to run?

1. First of all, you'll need to install [AFL++](https://github.com/AFLplusplus/AFLplusplus).
2. ```cargo run --bin main```

Now you can use CLI interface:

## Input and output format

The program consists of two main parts. The "tree" folder contains the implementation of the tree and the "main" folder contains the implementation of a program that reads instructions from stdin and then updates or dumps the tree. In detail, the main program is supposed to process the following commands:

### Insert

```
i <age:int> <name:string>
    For valid data, inserts an element into the tree with the specified data.
```

### Erase

```
e <age:int> <name:string>
    For valid data, erases the element from the tree that has data equal to the specified data.
```

### Check for the existence

```
c <age:int> <name:string>
    For valid data, prints "y\n" iff an equal element to the specified data is in the container already, "n\n" otherwise.
```

### Print

```
p
    Prints the elements in the tree. An element should be printed as: [<data>,<left>,<right>].
    After the entire dump, a newline character should be printed.

    <data> should be printed as: {"<age:int>":"<name:string>"}
           for example: {"5":"foo"}

    When left or right are null pointers, just "null" should be printed instead without the quotes.
        example: [{"5":"foo"},[{"3":"bar"},null,null],null]
        
    When the current tree is empty, just "null" (without the quotes) should be printed.
        example: null

```

### Reset

```
x
    Reset the internal state and start with a fresh and empty tree.
```


### Exit

```
q
    Exits the program
```


# Tests

1. First of all, you'll need to install [AFL++](https://github.com/AFLplusplus/AFLplusplus).
2. Run `cargo test`
3. Run `./test_main.sh`
4. Run `./test_sorted.sh`

# Fuzzing

1. First of all, you'll need to install [AFL++](https://github.com/AFLplusplus/AFLplusplus).
2. To fuzz my implementation use script `./afl.build.sh` to build and `./afl.sh` to fuzz.
3. I’ve not found any bugs during almost 10 hours of fuzzing:
![image](https://user-images.githubusercontent.com/32279961/233071116-68dd8229-3788-4a4b-b00e-5c165307ef14.png)
4. Output can be found in [/fuzz/output](/fuzz/ouput) directory
5. Also I fuzzed it for a couple of minutes with `AFL_USE_MSAN=1`. No errors found too. 
