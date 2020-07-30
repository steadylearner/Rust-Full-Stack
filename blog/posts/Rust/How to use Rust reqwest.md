<!--
  Post{
    subtitle: "Learn how to use it to substitute CURL",
    image: "code/Rust_r.png",
    image_decription: "Rust Image from the website",
    tags: "Rust reqwest GitHub CURL",
  }
-->

<!-- Steadylearner -->

[Steadylearner]: s-
[Steadylearner Github]: g-/steadylearner
[posts]: g-/steadylearner/Steadylearner
[Blog]: s-/blog
[Markdown]: https://www.steadylearner.com/markdown
[prop-passer]: https://www.npmjs.com/package/prop-passer
[How to write less code for links in markdown with React]:  s-/blog/read/How-to-write-less-code-for-links-in-markdown-with-React
[How to turn chars into binary and vice versa with Rust]: https://www.steadylearner.com/blog/read/How-to-turn-chars-into-binary-and-vice-versa-with-Rust

<!-- / -->

<!-- Link -->

[Rust]: https://www.rust-lang.org/
[JSON]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON
[What is Binary Data]: https://www.techopedia.com/definition/17929/binary-data
[Reqwest]: https://github.com/seanmonstar/reqwest
[CURL]: https://curl.haxx.se/
[git]: https://git-scm.com/

<!-- / -->

If you have used [CURL] before, you should know that you can print the page with
```curl
$curl https://www.rust-lang.org
```
and save it in your machine with a little POSIX CLI knowledge.
```
$curl https://www.rust-lang.org > rust-org.html
```
What we are going to learn in this post is to find how to use [Rust][Rust] to replace those comamnds. It won't be difficult for we already have [reqwest][reqwest] crate that can be used instead of **CURL**.

The final goal of this post is to use it to download blog posts saved at [Github][posts]. You may wonder why you should use it instead of [git][git] commands. But, There are some benefits of using programming languages such as Rust and Python for this purpose instead of **git** and **CURL** etc.

1. You can make some predefined **variables** and functions to save you from typing same things repeatedly.
2.  You can print some helper messages and you don't have to remember all requirements.
3. In case of **git**, you donwload .git files also and its size grows over time and you shouldn't need it just to download a single file to your machine.(You don't have to type your id and password also.)

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to use Rust][Rust]
2. [Rust Reqwest Crate][Reqwest]
---

You should already know how to write [Rust] code before you read on. I also want you to visit the [reqwest] GitHub repository to understand how to use it.

We will start from the [simple.rs](https://github.com/seanmonstar/reqwest/blob/master/examples/simple.rs) file  at the repository. I hope you read the other example files and test it before read on.

<br />

<h2 class="blue">Table of Contents</h2>

1. **How to print response.body** on your console with **Reqwest**
2. **How to save it** in your machine with **Rust**
3. How to use it in your **CLI**
4. **Conclusion**

---

<br />

## 1. How to print response.body on your console with Reqwest

I hope you already followed the examples at [reqwest]. You should have seen messages with .html format similiar to

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Rust programming language</title>
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <meta name="description" content="Empowering everyone to build reliable and efficient software.">
```

and there is no complexity here. But it isn't useful yet so we will tweak the code from the crate and we will use pages at


https://raw.githubusercontent.com/steadylearner/Steadylearner/master/post

instaed of  https://www.rust-lang.org.


The new code snippet for that is
```rust
extern crate reqwest;

fn main() -> Result<(), Box<std::error::Error>> {
    // 1.
    let prefix =  format!("{}", "https://raw.githubusercontent.com/steadylearner/Steadylearner/master/post/");
    // 2.
    let type_of_lang = format!("{}", "rust");
    // 3.
    let post_title = format!("{}", "How to turn chars into binary with Rust.md");

    println!("GET {}-{}", &type_of_lang, &post_title);
    let target = format!("{}{}-{}", &prefix, &type_of_lang, &post_title);
    let mut res = reqwest::get(&target)?;

    std::io::copy(&mut res, &mut std::io::stdout())?;

    Ok(())
}
```
What it intend to do are similar to the example at [reqwest].

But the differences are

1. We extract common parts from the location of each files to **variable** and we don't have to type it everytime with help from **Rust**. It can differ from user to user.
2. Each file used here as example has its type of language to differenciate themeselves from others(You can skip this part if you want.)
3. The process is similar to `1.` and it is our **payload**(It will be used with CLI later.)

If you execute this file you will see message that starts with

```
Post {
   title: "How to turn chars into binary and vice versa with Rust ",
   subtitle:  "Learn how to use Rust to decode and encode binary data.",
   image:  "/code/Rust.svg",
   image_decription: "Rust Image from the website",
   tags: "Rust binary chars stream",
   theme: "rust",
}
```
From now on, You can use **Rust** instead of **CURL** to show **response.body** on your console whenever you want with a little modification for your project.

<br />

## 2. How to save it

Showing results of the process at console is maybe the best way to help users to understand how packages work. But it isn't that useful if you couldn't find how to use it as variables and use them for your project.

In this process, we will learn how to do it with [reqwest][reqwest] and it will be easy with working examples.

The entire code will be
```rust
extern crate reqwest;
use std::fs::{write};

fn main() -> Result<(), Box<std::error::Error>> {
    let prefix =  format!("{}", "https://raw.githubusercontent.com/steadylearner/Steadylearner/master/post/");
    let type_of_lang = format!("{}", "rust");
    let post_title = format!("{}", "How to turn chars into binary with Rust.md");

    println!("GET {}-{}", &type_of_lang, &post_title);
    let target = format!("{}{}-{}", &prefix, &type_of_lang, &post_title);
    let mut res = reqwest::get(&target)?;

    // 1.
    let mut body: Vec<u8> = vec![];
    std::io::copy(&mut res, &mut body)?;
    println!("{:?}", &body);

    // 2.
    let characters: Vec<char> = body.into_iter().map(|x| x as char).collect();
    let result: String = characters.into_iter().collect();

    // 3.
    write("post.md", &result)?

    Ok(())
}
```
and there are some differences from the previous one.

1. We create empty variable **body** to hold stream data from res and use **println!("{:?}", body)** to verify **body** variable after being affected by  **std::io::copy** process.
2. You should have seen that it is still in **binary format**. So we have to convert it to String and save it to make it more **human readable** and **useful for** later uses.
3. We prepared `String` type data equal to what is inside [the original page](https://raw.githubusercontent.com/steadylearner/Steadylearner/master/post/rust-How%20to%20turn%20chars%20into%20binary%20with%20Rust.md) and what we need to do is just to save it with **write** method from **fs** module.

If you execute it you will see the **post.md** file saved in your current directory. We advanced the previous code that can **copy file** from the web.
<br />

## 3. How to use it in your CLI

I hope you followed the previous processes well. The example serves well for the purpose of this post. But it isn't that customizable yet. So we will include some code snippets to help you.

We will first include function `str_from_stdin`
```rust
// The goal of this function is to remove \n charatcer from the user_input
pub fn str_from_stdin() -> String {
    //
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input[..(user_input.len() - 1)].to_string();

    user_Input
}
```
inside the previous code snippet.

Then we will replace
```rust
let post_title = format!("{}", "How to turn chars into binary with Rust.md");
```
with example code below.
```rust
println!("What is the title of the post at GitHub?{}", bold.apply_to("(You don't need to type .md)"));
let type_post_title = str_from_stdin();
let post_title = format!("{}.md", type_post_title);
```
In the code above we also included **.md** extension for the GitHub Repostiroy is for **.md** files for blog posts.

I want you to personalize your own project referring to final code snippet below.

```rust
extern crate reqwest;
use std::fs::{write};

// inside the same file or others
pub fn str_from_stdin() -> String {
    //
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input[..(user_input.len() - 1)].to_string();

    user_input
}

fn main() -> Result<(), Box<std::error::Error>> {
    let prefix =  format!("{}", "https://raw.githubusercontent.com/steadylearner/Steadylearner/master/post/");
    let type_of_lang = format!("{}", "rust");

    println!("What is the title of the post at Github?{}", bold.apply_to("(You don't need to type .md)"));
    let type_post_title = str_from_stdin();
    let post_title = format!("{}.md", type_post_title);

    println!("GET {}-{}", &type_of_lang, &post_title);
    let target = format!("{}{}-{}", &prefix, &type_of_lang, &post_title);
    let mut res = reqwest::get(&target)?

    let mut body: Vec<u8> = vec![];
    std::io::copy(&mut res, &mut body)?;
    println!("{:?}", &body);

    let characters: Vec<char> = body.into_iter().map(|x| x as char).collect();
    let result: String = characters.into_iter().collect();

    println!("{:#?}", &result)
    write("post.md", &result)?

    Ok(())
}
```

<br />

## 4. Conclusion

Following this post, we could find the working Rust codes that can replace the role of **CURL** commmands mentioned in the intro of this post with  [reqwest].

The code snippet used here alone would not be so useful. I hope you to include it inside your projects.

**Thanks and please share this post with others.**
