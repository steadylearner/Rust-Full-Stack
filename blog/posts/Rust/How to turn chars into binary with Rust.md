<!--
  Post{
    subtitle:  "Learn how to use Rust to decode and encode binary data.",
    image:  "/code/Rust.svg",
    image_decription: "Rust Image from the website",
    tags: "Rust binary chars stream",
  }
-->

<!-- Steadylearner -->

[Steadylearner]: s-
[Steadylearner Github]: g-/steadylearner
[posts]: g-/steadylearner/Steadylearner
[Blog]: s-/blog
[Markdown]: https://www.steadylearner.com/markdown
[prop-passer]: https://www.npmjs.com/package/prop-passer
[How to write less code for links in markdown with React]: https://www.steadylearner.com/blog/read/How-to-write-less-code-for-links-in-markdown-with-React

<!-- / -->

<!-- Link -->

[Rust]: https://www.rust-lang.org/
[JSON]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON
[What is Binary Data]: https://www.techopedia.com/definition/17929/binary-data

<!-- / -->

When programming in [Rust][Rust] and others, I found that it is important to know how to **decode** and **encode** binary datas.

You can help the machine to understand the data you pass in the **binary** format and you can also read them by converting them into **chars**(characters) and make it work for the important processes such as writing files, receivng data from the web etc.

If you are a beginner in **Rust**, it may not be easy to find how to do them in the first place. So I wanted to share the code to help you. You will find that is is easy  with the working examples.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to use Rust][Rust]
2. [What is Binary Data][What is Binary Data]
---

You should already know how to write [Rust][Rust] code before you read on. I also want you to understand what is [binary data][What is Binary Data] before. It will help you to understand the importance of knowing how to deal with it.

If you already have experience in [JSON][JSON] but not familiar with this topic, you may compare what you will learn here with `JSON.parse` and `JSON.stringfiy` to decode and encode **JSON** type data.

<br />

<h2 class="blue">Table of Contents</h2>

1. How to turn **chars** into **binary data** with Rust
2. **Vice Versa**
3. **Conclusion**

---

<br />

## 1. How to turn chars into binary data with Rust

It was not so difficult to find how to do it with **Rust**. For it has very simple way to do that already.

In the code snippet below
```rust
fn main() {
    let bianry:&amp;[u8] = b"rust";
    println!("{:?}", binary);
    // result in [114, 117, 115, 116]
}
```
what we need to do was just to put [`b`](https://medium.com/r/?url=https%3A%2F%2Fdoc.rust-lang.org%2Freference%2Ftokens.html%23byte-and-byte-string-literals) in front of the `&amp;str` type data(**"rust"**) and **Rust** does the process to convert **chars into binary** instead of you. By far, it was the easiest way comparing to how to do that in other programming languages.

For example, in **JavaScript**
```js
const rust = "rust";
const binary = rust.split("").map(x => x.charCodeAt());
console.log(binary);
```

You can see that we don't need to use methods such as `split` and `map` for our purpose when we use Rust for the same purpose.

What we need was just a single character `b` and nothing more.

<br />

## 2. Vice Versa

For we learnt how to encode our data into binary format in the previous part, we will learn how to decode it. Comparing to the previous process, it will be a little bit more complicated.

The entire code example for this part is
```rust
fn main() {
    let binary: Vec<u8> = vec![114, 117, 115, 116]; // 1.
    let characters: Vec<char> = binary.into_iter().map(|x| x as char).collect(); // 2.
    let result: String = characters.into_iter().collect(); // 3.
    println!("{:?}", &amp;result); // 4
}
```

You should have found that we need much more code to make **chars**(characters) from **binary** datas.

1. In the previous part, we already found that **binary data** type equivalent to **"rust"** is `[114, 117, 115, 116]` so we will use it for the purpose of this process is to **decode** instead of encode.
2. We first define type of variable with `Vec<char>` for **characters** variable for **Rust** is typed language and you should help it to understand what you want to do. We need **char** type instead of **u8** type after passing this process with **binary.into_iter().map(|x| x as char).collect()**. Notice that difference to understand this code.
3. We already made vector of **chars** from the vector of u8(**binary data**) in the previous process. But it will not be easy to understand what data was in this format. So we turn it into **String** type to make it more human readable.
4. We use **println!** macro to show the result to our console and you will see "rust" when you execute this code in your machine.

You may found that it is not so easy. To understand this code completly, you should understand what each parts of it does with details.

For that, You can use `$rustup doc --std` to find the documentation easily in your local machine.

Because the process used many times, it will be better for you to save the process inside function or macro and use it again whenever you want.

<br />

## 3. Conclusion

The code used here is simple. However, understanding it may not be that easy if you are new to system programming. But it will be useful if you just wanted to know how to **decode** and **encode** binary data in **Rust** and find working codes.

You can contribute to this post and other posts at [Github repository for blog][posts].

**Thanks and please share this post with others.**

