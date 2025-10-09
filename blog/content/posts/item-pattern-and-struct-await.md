+++
title = "Item Patterns and Struct Await"
date = "2025-10-09"
author = "Noratrieb"
tags = ["rust", "language-design"]
keywords = ["design"]
description = "Bringing even more expressiveness to our items"
showFullContent = false
readingTime = true
hideComments = false
draft = false
+++

# The Inconsistency Problem

Quite some time ago, I wrote [a blog post about the idea of `struct else`](./item-patterns-and-struct-else.md), which has been widely regarded as one of the most insightful pieces of Rust language design in the last few years[^insightful].
If you have not read it, it's **strongly** recommended to first read it.

But more recently, it has become clear that there are some serious problems around items that cause this to not work properly.

This example non-deterministically results in an error about unresolved items.

```rust
struct Test;
fn main() {
    let x = Test;
}
```

```
error[E0425]: cannot find value `Test` in this scope
 --> <anon>:3:13
  |
3 |     let x = Test;
  |             ^^^^ not found in this scope
```

This points at a concerning inconsistency in the Rust compiler. Is it a bug? Not really, it's just a lack of expressivity in the language.

Let's go through this error step-by-step and see exactly why this happens.

First, `struct Test` gets evaluated by the compiler.
It initializes the `Test` struct in the compiler, but the problem is that this initialization happens *asynchronously*.
This means that when we are compiling the `main` function, it might not have finished, causing the item to be missing.

We now understand how this error comes to be, but how can it be fixed?

## Asynchronous Programming

Before we can understand the ideal solution to this problem, we need to first do a deep dive into asynchronous programming in general.

The "async" in asynchronous programming is derived from the word "asynchronous".
Asynchronous programming refers to a particular style of programming where the execution flow is asynchronous.
This means that statements are executed asynchronously.
This asynchronousity can be difficult to work with, which is why it is generally tamed by awaiting.
Rust supports asynchronous programming with its `async` keyword, which is the shortened form of the word "asynchronous".
Rust has many keywords shortened like this, for example `fn` for "function" and `trait` for "traitor".

Now that we know what asynchronous programming is, we can use our knowledge of it to tame our struct declarations.

## Awaiting programming

We can use the `.await` keyword in Rust to await an async function, taming its asynchronousity into something much easier to understand.
It's like if you tame a bear and make it meow. It will still eat you though.

If we use the `.await` keyword on a struct declaration, we can tell the compiler to await the struct declaration, ensuring it will be finished initializing properly.

```rust
struct Test.await;
fn main() {
    let x = Test; // works!!
}
```

This is great, and fixes this long-standing issue with items in Rust.

But we can take this further.

## Generative Awaiting

In Rust, we often want to generate code for some interface that is declared on the web.
It could be a W3C IDL, a Wayland Protocol XML or even a programming language specification like the Rust specification.

Today, you need to download this file in a build script and generate the code from there.
This works great and is extremely flexible, but there are some security concerns with downloading a file during the build.
To avoid this, we should better integrate this with the language, and thanks to `.await` on structs we can.

If we can use `.await` on anything, we can download a file right off the internet in our Rust source code!

Combine this with our previous struct else logic, and we can generate code right in the items:

```rust
impl std::net::http::Download<"https://noratrieb.dev/protocol.xml"> for Download {}
struct Download.await;

struct Parsed {
  name: String,
  field_type: String,
}

impl std::xml::Parse<Download> for Parsed {}

struct Protocol {
    [Parsed.name]: String,
    my_field: [Parsed.field_type],
} else {
    compile_error!("invalid xml");
}
```

As we can see here, we now `.await` the declaration of the `Download` struct, which will automatically download a `protocol.xml` from my website[^protocolxml].
We can then parse the XML using the novel `std::xml` module[^libxml], and then create a new struct declaration depending on the results of that.
If the XML is malformed, we can make use of `struct else` to error out in this case.

## Conclusion

We were able to harness the power of asynchronous programming and `.await` to not only fix a long-standing bug in the Rust language, but also significantly enhance the power of code generation in the Rust programming language.
This shows the flexibility of the programming style, which can be used for many other things, like implementing embedded control systems for killer drones or implementing a high performance webserver for a toy store. Or serving shitposts over the web. Soooo many use cases.


[^insightful]: Supposedly. People have said "still shocked this has not been stabilized yet" about this.
[^protocolxml]: meow.
[^libxml]: It is suggested to use `libxml2` to implement this module.