
## Reddit Question [lu27lz](https://www.reddit.com/r/rust/comments/lu27lz/im_moving_a_big_array_around_too_much/)
### General Idea
I have been toying around with a small project thats inspiration came from Brilliant.org's math fundamentals. The general Idea is you have a 3x3 array of different number from 1 to 10 that when added up equal 12 like so :

```
    [4] [5] [3] 12
    [6] [0] [9]
    [2] [7] [8]
    12

```

### Where I'm at

[My github]("https://github.com/jackrizza/compare_list")

I have built this in python but it is very slow and has scaling issues so, I thought I'd try writing it in rust. I have a comparator written that takes two array's and make sure it hasn't been used before. which later on I will run on the ```new_abc_array()``` to slim it down.

I also have a recursize function that keeps iterating over ```new_abc_array()```. 

### My problem

````

> Cargo Run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/compare_list`
COMPARE TEST : false
[[0, 0, 1], [2, 2, 2], [3, 3, 3]]
[[3, 3, 5], [0, 0, 2], [1, 1, 1]]
[[4, 4, 7], [0, 0, 2], [1, 1, 1]]
[[5, 5, 9], [0, 0, 2], [1, 1, 1]]
[[6, 6, 11], [0, 0, 2], [1, 1, 1]]
[[7, 7, 13], [0, 0, 2], [1, 1, 1]]
[[8, 8, 15], [0, 0, 2], [1, 1, 1]]
[[9, 9, 17], [0, 0, 2], [1, 1, 1]]
[[10, 10, 19], [0, 0, 2], [1, 1, 1]]
[[11, 11, 21], [0, 0, 2], [1, 1, 1]]

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    59808 abort      cargo run
````

I believe the reason for the thread 'main' has overflowed error is because I am constantly making a new structure in the recurisize function called ```General_structure``` which is writing ```new_abc_array``` to the structure to be passed back to the recurize array.

### The question

Am I right about this? and how do I store the vector ```new_abc_array``` safely so that I can just use a pointer.

thanks to anyone that is will to look I have been programming in rust for about a year now I still have to learn alot and I know the current code is hack but I will clean it up after I have some that can run.
