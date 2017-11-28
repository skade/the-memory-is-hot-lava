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
    let number = 42
}
```

The resulting memory might look like this:

```
|  |  |  |  |  |  |42 |  |  |  |  |  |
 0  1  2  3  4  5  6  7  8  9  10 11
```

There's two things to note here: the number occupies memory slot 6. It fully occupies it. The information that 42 is labeled "number" doesn't exist in this image - it isn't necessary. Also note that we ended up in slot 6: this is the slot we got assigned for the number.

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