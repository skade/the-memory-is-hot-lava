# The Memory is Lava

A gentle introduction into memory, memory safety, ownership and borrowing in Rust.

## A word about memory

Memory is one of the most imporant parts of any computer. In memory, all data is stored and kept in between computations. Yet, in contrast to most other parts of the machine, it follows a relatively easy model.

For the purpose of this introduction, let us imagine our computers memory as a number of boxes of equal size, set up next to each other. Each of them gets a number.

```
|  |  |  |  |  |  |  |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

We call those numbers "addresses", as we can use them to address any of these boxes individually.

From there, we can start working: we can use all these boxes to store things, read things or to make them empty again.

This is simple, but surprisingly close to how real-world memory is seen from a program.

The complexity lies in its memory.

## Dangers in Memory

There's already a danger in this simplicity: memory can be misused. Let's have a look at this simple procedure:

* *Write the number 2 to address 0*
* *Read from address 1*
* *Print the number read*

What does it do? Note that we don't read from the address that we wrote to. That means we don't know. Without any further info, we can't predict what's in address 1. It might be unused, so it contains "nothing" (how do you print nothing?), it might also contain a number someone else stored there. And who are we to print someone elses number?

Both of these things we don't want. Ensuring that neither of them can happen is an aspect of something we call *memory safety*.

This text introduces you to memory safety and how it can be ensured. All examples are in Rust.

## The Memory is Lava

Before we start programming, let's have a look at a nice trick. We just found out that reading from an address that doesn't contain anything can never return something meaningful. We might as well stop any program that does this right in its tracks. Indeed, this is what most modern operating systems do: they will abort the program if that happens.

They do this playing a game of "the floor is lava" with you: if you miss and accidentally read a memory address that is not in use, you lose. For that reason, we fill our whole memory with lava before we start. We then have to _ask_ the operating system to make us an island in that lava that we can use. For the sake of this tutorial, the operating system is a very happy player of [Populous](TODO: Link).


```
TODO: Find Iconography for lava
|  |  |  |  |  |  |  |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

You can see this effect in many modern programs: whenever your operating system mentions that a program "stopped working" or "crashed", this is exactly what happened. Someone jumped in lava.

## Simple Interactions with Memory

But first things first: lets get some data into memory. Let's start with something simple, a simple number. In Rust, we just have to bind an number to a variable to introduce it to memory.

```rust
fn main() {
    let number = 42;
}
```

The resulting memory might look like this:

```
|  |  |  |  |  |  |42 |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

There's two things to note here: the number occupies memory slot 6. It fully occupies it. The information that 42 is labeled "number" doesn't exist in this image - it isn't necessary. Also note that we ended up in slot 6: this is the slot we got assigned for the number.

Let's remove that number again:

TODO: explain drop better
```rust
fn main() {
    let number = 42;
    drop(number);
}
```

Once `number` ceases to exist in our program, it will be removed from memory:


```
|  |  |  |  |  |  |  |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

## Complex data

Let's try something bigger: let's take a point with 2 coordinates and store that in memory. The process is the same as before, just that the notation to get a `Point` is a bit different:

```rust
struct Point {
    x: i64
    y: i64
}

fn main() {
    let p = Point { x: 56, y: 23 };
}
```

The resulting memory may look like this:


```
|  |  |  |  |  |  |  |56|23|  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

Again, note that our memory doesn't have an idea which of the two cells hold `x` and which holds `y`. This is up to our programming language. This is called "memory layout". Much like variables, the programming language itself decides how to write a structure to memory. The simplest way to do this is by just writing the fields to memory in the order of appearance, which happens here.

Much like variables, field names are a concept easing development for the developer, the programmer doesn't know about it.

### Aside: Offsets

We just decided that if we store a `Point` in memory, it will write both fields to memory, `x` before `y`. This gives us an interesting option: when answering the question "Where is y?", we can give two answers. An absolute one ("`y` is at slot 8) or a relative one ("`y` is one slot above `x`"). In the second case, we say that `y` has an _offset_ of 1.

Offsets are not necessarily always of the size 1, this depends on what they describe. We'll see an example soon.

## Tuples

There's another classic structure that holds multiple values in many programming languages: Tuples. A tuple is a group of values, but they are not necessarily named. In Rust, the notation is:

```rust
fn main() {
    let point = (56,23); 
}
```

Tuples are much more generic then structs (for example, their fields are called `0` and `1`). But let's look at how they end up in memory:


```
|  |  |  |  |  |  |  |56|23|  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

Just the same as our struct. This shows us that semantically different things in our programming language might end up being just the same in memory. This doesn't mean that you should use them interchangeably, but it means that we have choice here.

## Arrays

Another way to combine two simple values like numbers is to add them to consider them a list. There's many implementations of these structures, but we're interested in a simple one: Arrays. This is what an `Array` with 2 elements in Rust looks like:


```rust
fn main() {
    let point = [56,23]; 
}
```

And, this is what it could look like in memory:

```
|  |  |  |  |  |  |  |56|23|  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

The same. As long as we know the length of the array, we can just work as before: we now that the array starts at slot 7, and has two elements. So we can find the second element just by adding an _offset_ of 1 and reach slot 8.

What happens if we try an _offset_ of 2? There would be lava there, _but_, the compiler knows that and doesn't let go there.


```rust
TODO: example
```

The compiler knows about the _size_ of what is stored in _point_ and only allows us to access this range. This is the first time we encounter _memory safety_ directly. The compiler saves us from using an insecure offset.

## Complex arrays

Now, what if we want to store multiple of the `Point` values? Simple, let's create an array of them!

```rust
fn main() {
    let points = [ Point { x: 56, y: 23}, Point { x: 40, y: 50 } ]; 
}
```

Let's see what that looks like in memory:


```
|  |  |  |  |  |  |  |56|23|40|50|  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

Nice, this still looks very effective. The reason behind this is simple: we know the _layout_ and the _size_ of `Point`s (two integers, x is first, y comes second), and we know that we have an array of two of these. The first point is at slot 7, and the second one is at 7 _plus and offset of 1_. Remember how we said that an offset is not always one memory slot away? It's the case here, because we know how large Points are, so we know that the _offset_ between elements is 2 slots. This is useful, because the arithmetic behind accessing array elements stays the same.

## memoryslotspointingatthings.tumblr.com

TODO: find a better section title

Which brings us to the next question: what happens when we want to hand out one of these `Point`s to another part of the program, but don't want to send the whole array over? This is were pointers come into play.

While it's usually impolite to point at other people, pointing at data is a thing that should be practiced frequently and often.

Pointers use a simple semantic: they are memory addresses stored in memory slots. Consider a pointer to the second element of our array:


```
|  |  |9 |  |  |  |  |56|23|40|50|  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

The pointer is stored in slot 2, _pointing_ to slot 9. This is where our second `Point` lives. Through this pointer, we can still access `x` and `y`. For following the pointer to `x`, we just follow it. For `y`, we follow it with an _offset_ of 1 slot. As programmers hate having two different operations for all this, we can also say that `x` is at the location that the pointer points to, plus and _offset_ of 0.

# The evil pointer

TODO: good programatic examples 

Now, consider this:

```
|  |  |9 |  |  |  |  |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

The pointer at slot 2 points to a `Point` at location 9, but there's nothing there.

See the issue? We have a pointer to a memory location, but the location is currently empty. It's lava. What happens if we try to access memory there? We lost the game, there kernel kills our program.

There's another, more evil case:  


```
|  |  |9 |  |  |  |  |a |b |c |d |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

We expected location 9 and 10 to contain a point, but instead, someone removed the array and put text there in the meantime. This is a case of _memory corruption_. Our program expects a `Point` there, but instead, there's characters of a text there. What will happen? We don't know. Our program will continue running with weird data.

This can be very dangerous, as it can lead to security issues.

## Are pointers unsafe?

Rust has a data type called "raw pointer". A raw pointer to a `Point` has the notation `*const Point`. Because we never know what's behind a pointer, Rust calls following a raw pointer "unsafe".

TODO: unsafe dereferncing of a pointer

But does that have to be like this? Consider the sequence of memory operations:

1) Storing our Point array 

```
|  |  |  |  |  |  |  |56|23|40|50|  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

2) Storing a pointer to our second point

```
|  |  |9 |  |  |  |  |56|23|40|50|  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

3) Removing the pointer

```
|  |  |  |  |  |  |  |56|23|40|50|  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

4) Removing the Point array

```
|  |  |  |  |  |  |  |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

At any point in this sequence, all data is valid. Especially following the pointer is sound.

Now consider this sequence:


1) Storing our Point array 

```
|  |  |  |  |  |  |  |56|23|40|50|  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

2) Storing a pointer to our second point

```
|  |  |9 |  |  |  |  |56|23|40|50|  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

3) Removing the Point array

```
|  |  |9 |  |  |  |  |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

4) Removing the pointer

```
|  |  |  |  |  |  |  |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

This is dangerous: if anyone were to read what's behind the pointer between 3) and 4), we're landing in lava.

There's an connection between the memory slot 2 and the memory slots 7-10, but it only happens on our program level: slot 2 is not meaningful without the other ones. The hardware (our memory bank) doesn't care. It can't, it is not in its feature set.

But that means, we can _fix that_ in our programming language.

## Borrows

Rust makes pointers safe by introducing the concept of "borrows". This is purely a language feature happening during compilation of the program. Borrows are only valid while the data they _borrow from_ is valid. If you break this rule by any means, you will get a _lifetime error_. Rust knows when you are trying to keep a pointer around longer then the thing it points to.

```rust
fn main() {
    let points = [ Point { x: 56, y: 23}, Point { x: 40, y: 50 } ];
    let second_point = points[1];
    
    drop(points);
    
    *second_point;  
}
```

TODO: compiler error

Oops!
