<!--
  Post{
    subtitle: "Learn how to set up Rust project for blog.",
    image: "/page/steadylearner-blog.png",
    image_decription: "Steadylearner Blog Page",
    tags: "Rust blog  Steadylearner",
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
[Diesel]: http://diesel.rs/guides/getting-started/
[Postgresql]: https://www.postgresql.org/
[Rocket]: https://rocket.rs/
[React]: https://reactjs.org/

<!-- / -->

If you wanted to write blog with Rust and couldn't figure out how to do that, You may start with this post. I want to share my knowledge from building [Steadylearner][Steadylearner]. You wouldn't need much expereience in **Rust** because you will get completely working project at the end of each post.

What we need to do to build [Rust Blog][Blog] are

1. **Rust CLI** to handle database for blogs and users
2. Routes for **static files and pages**
3. **JSON web service** to send the datas used for blogs
4. Routets for **dynamic pages**

For **routing** and **JSON Web Service**, we will use [Rocket](https://rocket.rs/) and Frontend will be [React][React].

You may think that there are so many things to learn but don't worry because I am planning to make each project and project work on their own.

In this post, we will learn how to build **Rust CLI** for blogs fist.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to use Rust][Rust]
2. ["Hello World" with Rust](https://github.com/nrc/r4cppp/blob/master/hello%20world.md)
3. [Postgresql][Postgresql]
4. [Rust Blog CLI][Diesel]

---

If you are a total beginner of Rust, you can start from building "Hello World" application with **Rust***. Then, you can start with [Rust Documentation][Rust] to learn how to use it.

You can use `$rustup doc --std` in your console. Whenever you have doubt in some Rust API. If you want to save your time.

You can save command similar to this in your ~/.bashrc

```shell
#Rust
alias rustapi="rustup doc --std"
```

Then, type `$source ~/.bashrc` and you can open Rust API Documentation with `$rustapi`.

It is necessary to have any expereience in [SQL languages][Postgesql]. It will help you to follow [Rust CLI Blog with Diesel][Diesel] better and other posts relevant to [blog] later.

Before you read on, I hope you already made your Rust CLI Blog work. If you find it difficult, just don't give up. You can make it eventually.

If you see `cc failed exit code 1`, while installling **Diesel CLI**,

You can use `$sudo apt install libpq-dev libmysqlclient-dev` to solve the problem.(It is relevant **Cargo Package Manager**)

<br />

<h2 class="blue">Table of Contents</h2>

1. **SQL** for blogs and users with **Diesel CLI**
2. How to improve **CVRUD** from
3. Some helper fuctnions to save your time
4. Conclusion

---

<br />

## 1. How to print response.body on your console with Reqwest

I hope you already followed the examples given before. You should have seen messages with .html format similiar to

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Rust programming language</title>
    <meta name="viewport" content="width=device-width,initial-scale=1.0">
    <meta name="description" content="Empowering everyone to build reliable and efficient software.">
```

and there is no complexity here. But it isn't useful ye so we will tweak the example code from the crate from now on and we will use pages at

```
https://raw.githubusercontent.com/steadylearner/Steadylearner/master/post
```
instaed of  `https://www.rust-lang.org`.


The entire code snippet will be
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
The code snippet is not so different from the example at [reqwest][reqwest].

The differences are

1. We extract common parts from the location of each files to **variable** and we don't have to type it everytime with help from **Rust**. It can differ from user to user.
2. The files used here as examples has its type of language to differenciate themeselves from others. You can skip this part if you want. It is used here because it will be used in other posts also.
3. The process is similar to `1.` and it is our **payload**(It will be used with CLI later.)

If you exectue this file you will se message that starts with

```md
<!--
 Post{
   title: "How to turn chars into binary and vice versa with Rust ",
   subtitle:  "Learn how to use Rust to decode and encode binary data.",
   image:  "/code/Rust.svg",
   image_decription: "Rust Image from the website",
   tags: "Rust binary chars stream",
   theme: "rust",
 }
-->
```
With this code, you can use **Rust** instead of **CURL** to show **response.body** on your console whenever you want with a little modification for your project.

<br />

## 2. How to save it

Showing results of the process at console is maybe the best way to help users to understand how packages work. But it isn't that helpful if you couldn't find how to use it as varaibles and use them for your project.

In this process we will learn how to do it with [reqwest][reqwest] and it is easy for we already prepared code for that in [How to turn chars into binary and vice versa with Rust][How to turn chars into binary and vice versa with Rust].

We will include it inside the  previous code snippet and use `fs` from **Rust** to replace the role of  `> rust-orn.html` before.

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
and there are some differences from the previous code.

1. We create empty variable **body** to hold stream data from res and use **println!("{:?}", body)** to verify **body** variable after being affected by  **std::io::copy** process.
2. You should have seen that it is still in **binary format**. So we have to convert it to String and save it to make it more **human readable** and **useful for** later uses.(You should have read [the post][How to turn chars into binary and vice versa with Rust] to understand how it works.)
3. We prepared `String` type data equal to what is inside [the original page](https://raw.githubusercontent.com/steadylearner/Steadylearner/master/post/rust-How%20to%20turn%20chars%20into%20binary%20with%20Rust.md) and what we need to do is just to save it with **write** method from **fs** module.

If you execute it you will see the **post.md** file saved in your current directory. We advanced the previous code that can **copy file**** from the web instead of  `> post.md`.

<br />

## 3. How to use it in your CLI

I hope you followed the previous processes well. The example serves well for the purpose of this post. But it isn't that customizable well yet. So we will include some code snippets to help you.

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
println!("What is the title of the post at Github?{}", bold.apply_to("(You don't need to type .md)"));
let type_post_title = str_from_stdin();
let post_title = format!("{}.md", type_post_title);
```
In the code above we also included **.md** extension for the GIthub Repostiroy is for **.md** files for blog posts.

I want you can personalize your own project referring to final code snippet below.

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

Following this post, we could find the working codes that can replace the role fo **CURL** commmands mentioned in the intro of this post.I hope you could also learn how to use [reqwest][reqwest] and organlize your project.

The code snippet used here alone would not be so useful. I hope you to apply the process  used here to another project such as downloading posts and uploading posts directly from [Github][posts].

You can contribute to this post and other posts at [Github repository for blog][posts].

**Thanks and please share this post with others.**
