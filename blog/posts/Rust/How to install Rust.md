<!--
  Post{
    subtitle: "Learn how to install Rust and more",
    image: "post/Rust/how-to-install-rust.png",
    image_decription: "Made with CSS by Steadylearner",
    tags: "Rust How install code",
  }
-->

<!-- Link -->

[Rust]: https://www.rust-lang.org/
[Rust Getting Started Guide]: https://www.rust-lang.org/learn/get-started
[Rust IDE Support]: https://www.rust-lang.org/tools
[How to learn Rust]: https://doc.rust-lang.org/book/
[Rust API]: https://doc.rust-lang.org/std
[Rust By Example]: https://doc.rust-lang.org/stable/rust-by-example/
[Steadylearner]: https://www.steadylearner.com
[How to deploy Rust Web App]: https://medium.com/@steadylearner/how-to-deploy-rust-web-application-8c0e81394bd5?source=---------9------------------

[EssentialC]: http://cslibrary.stanford.edu/101/EssentialC.pdf
[The C Programming Lanugage]: https://www.google.com/search?q=the+c+programming+language

[prop-passer]: https://github.com/steadylearner/prop-passer
[react-easy-md]: https://github.com/steadylearner/react-easy-md

<!-- / -->

<!-- Post -->

<!-- / -->

If your goal is just to install Rust language in your machine, the fastest way will be visit [Rust Getting Started Guide].

(You can skip intro below and move to prerequisite if you want to save your time without intro.)

It has been more than a year, I started learning [Rust] Programming lnaguage.

I liked [Firefox Developler Editon](https://www.mozilla.org/en-US/firefox/developer/) for it is definitely faster than normal firefox version and some other browsers I had.

(Well, my laptop is more than half ten year and internet speed here is far from fast so I think that I felt more difference.)

Later, I found that it was written in [Rust] and started having interest in it.

I spent a month just to read the documentation for [Rust] and installing it was not so easy.

Time passed and Rust development ecosystem became better. I thought that it is time to share my experience in Rust with ohters.

What you will find in this post will be the minimum information you will need to start to develope with Rust.

It is documentation to help myself later but hope this post helpful for others also.

I will update this post when there is something useful for this topic.

(Linux Ubuntu 18.04 OS is used here and you can use this post just for reference if you use another system.)

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [Rust]
2. [Rust Getting Started Guide]
3. [Rust IDE Support]
4. [How to learn Rust]
5. [Rust By Example]

---

I hope you already visited [Rust Getting Started Guide]. It has almost all information to install [Rust] in your machine. But I want you to follow this post briefly to save your time and find the process better.

I will explain how to set up Rust development enivironment in your IDE such as VSCode later. Before that, I want you to verify the list of them at [Rust IDE Support] page.

If you want more practical examples, you may visit [it](https://rust-lang-nursery.github.io/rust-cookbook/) later.

It may not easy to learn [Rust] if you haven't known system languages before.

I hope you read some documenation for C such as [EssentialC] and [The C Programming Lanugage] before or after you learn [Rust] and compare them.

<br />

<h2 class="blue">Table of Contents</h2>

1. Rustup and Cargo
2. "Hello World" with Rust
3. Cargo Edit
4. Rust Language Server(RLS)
5. Conclusion

---

You may not need 3 and 4 if you are just starting with Rust.

<br />

## 1. Install Rustup and Cargo

If you visit the [Rust Getting Started Guide], the page will show the message similar to

> It looks like you are running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in
> your terminal, then follow the on-screen instructions.

I use Linux ubuntu 18.04 at this moment and the page showed command below for me.

```console
$curl https://sh.rustup.rs -sSf | sh
```

You can just copy what you saw and paste it on your console.

It will show message similar to this

```console
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

You won't need other options other than "Proceed with installation" so just press [Enter].

It will start installation and it may take long.

**So find something to do for a while or take a break.**

After long wait, it will show message similar to this.

```console
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'

Rust is installed now. Great!

To configure your current shell run source $HOME/.cargo/env
```

You can restart your machine at this point if you want. Otherwise do it at the end of this post. It will help paths for Rust toolchain to work well.

Follow the instruction and type `$source $HOME/.cargo/env` in your CLI because we want to do more than installing Rust.

(Type the same command if you had to use another console before you reboot your system.)

Then `$rustup` to verify it is installed well and it will show message similar to this.

```console
The Rust toolchain installer

USAGE:
    rustup [FLAGS] <SUBCOMMAND>

DISCUSSION:
    rustup installs The Rust Programming Language from the official
    release channels, enabling you to easily switch between stable,
    beta, and nightly compilers and keep them updated. It makes
    cross-compiling simpler with binary builds of the standard library
    for common platforms.
```

There are various options but learning how to use it will be primary point after you install it.

For that, You can first use `$rustup doc --book` to start to learn Rust. You should have seen that in your console output and the content for it will be similar to [How to learn Rust] page.

Then, you may type `$rustup doc --std` and it will show the same content for [Rust API] at your local machine.

You will need it later to find information for specific Rust API. So you can optionally make shortcut(alias) for that with bash files.

Type `$vim ~/.bashrc` in your CLI or use your IDE then type this at the end of it.

```sh
alias rustapi="rustup doc --std"
```

and save it.

Then, type `$source ~/.bashrc` and `$rustapi` in your CLI. With this simple modification, you can type `$rustapi` to find the documentation for Rust instead of searching and visiting [Rust API] page whenever you have doubts.

(If you want more features for rustup, you can start with [rustfmt](https://github.com/rust-lang/rustfmt) and [rust clippy](https://github.com/rust-lang/rust-clippy) with `$rustup component add rustfmt clippy` and it won't take long.)

If you read the install process, you can see that cargo is also installed. You can think it similar to NPM or Yarn if you are familar with JavaScript. It helps you easily build and manage your Rust projects.

Type `$cargo` on your console to verify it installed.

It will show message similar to this

```console
USAGE:
    cargo [OPTIONS] [SUBCOMMAND]

See 'cargo help <command>' for more information on a specific command.
```

It will not be easy to know how to use it at first glance.
So you may read [documentation](https://doc.rust-lang.org/cargo/index.html) first.

<br />

## 2. "Hello, world" with Rust

For we already have rustup and cargo installed, we can start our Rust app easily.

Type `cargo new --bin hello_world` and it will show

> Created binary (application) 'hello_world' package`

It should have created `Cargo.toml` and `main.rs` files for you also.

```console
[Cargo.toml]
├── Cargo.toml
└── src
    └── main.rs
```

You may think **Cargo.toml** file as package.json for JavaScript.
Inside your main.rs file you will see that **Rust developers** already prepared your first "Hello, world" programm for you.

```rust
fn main() {
  println!("Hello, world!");
}
```

So you don't even need to search how to code "Hello, world" programm in Rust with help from them.

You can just refer to it and `$cargo run` or `$cargo run --release` and it will show you.

(You could also use `$cargo c` to test Rust code before you compile and run them with those commands. It is omitted because examle is very simple.)   

```console
cargo run
     Running `target/debug/hello_world`
Hello, world!
```

If you tested both commands, `$cargo run --release` is much faster because it makes **optimized code without debuginfo**.

You made it. You are ready to start to code with Rust.

If you want to test it with more advanced example, you may clone the Steadylearner Chat GitHub repository with

`$git clone https://github.com/steadylearner/Chat.git`.

Then, `$yarn` in /static/chat folder to download NPM packages and `$cargo run` for Rust crates in ws_rs_with_rocket folder. 

It will show you the chat app with Rust Backend at http://localhost:8000/chat.

[The blog post to explain how to use it](https://medium.com/@steadylearner/how-to-start-rust-chat-app-499a194d0820) is mentioned at [This week in Rust](https://this-week-in-rust.org/blog/2019/05/21/this-week-in-rust-287/) and you may find it and the site useful also.

If you want to test Web development with Rust, you may refer to [How to start Rust Web Application](https://medium.com/@steadylearner/rust-web-app-1-setup-1c24c2c9ec66) also.

<br />

## 3. Cargo Edit

When you become familiar with Rust, you may want to find how to manage packages(crates in Rust) better, for that, you can install [cargo-edit](https://github.com/killercup/cargo-edit) pacakges if you want.

It requires installing some **nightly** features for Rust.

('nighlty' is to show they are with more advanced and experimental features but not officially published yet.)

You can do that with

1. `$rustup toolchain install nightly`
2. `$rustup component add rustfmt --toolchain nightly`
3. `$cargo install cargo-edit`

(`$sudo apt install pkg-config` first if you use Linux Ubuntu 18.04 or have error messages relevant to it.)

It may take a very long time. So please find other thing to do first.

After this process, you can mangage your crates for cargo.toml files easily with its features.

For example, Type `$cargo add <crate_name>` and you will see `<crate_name> v<latest_version_number> to dependencies` message at your console.

Then, in your cargo.toml file.

```toml
[dependencies]
<crate_name> = <latest_version_number>
```

is attached.

You can remove the crate with `cargo rm <crate_name>` and it will be also removed from your cargo.toml file.

I want you test it with Rust offical web framework [tide](https://github.com/rustasync/tide) or other [crates](https://crates.io/).

<br />

## 4. Rust Language Server(RLS)

If you want to use Rust with your IDE , you may refer to documenation for [racer](https://github.com/racer-rust/racer) for code completion and [Rust Language Server](https://github.com/rust-lang/rls) with various funcitonalities.

To use RLS you should install racer first and what you need are

1. `$cargo +nightly install racer` in your CLI. It may take long.
2. `$rustup component add rls rust-analysis rust-src` to install rls relevant features.

**rls** is made to be used with IDE such as VSCode, Vim etc according to the documenation. You can find if there are one that support your IDE at [Rust Get Started Page](https://www.rust-lang.org/learn/get-started).

Personally, I use VSCode and Vim and thought [VSCode Rust(rls) extensions](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) useful. So I will use it as an example for this post.

(You may find various extensions for Rust in VSCode. But I hope not to install other extensions except Rust(rls) before everything works fine becasuse some of them are not very compatible with others.)

After you install it, you should see RLS [Building] and other messages with crate names in your project.

You should wait for until it becomes RLS only like this image.

![rls complete](https://www.steadylearner.com/static/images/post/Rust/rls-complete.png) 

After that, you can test

```rust
fn main() {
println!("Hello, world!");
}
```

and format it with shortcut **[ctrl+shif+i]**.
Then, your editor will format it instead of you like the code snippet below.

```rust
fn main() {
    println!("Hello, world!");
}
```

and you are ready to use Rust in your IDE also.

If you find problem with the process, you can

1. **[ctrl+shift+p]** in your **VSCode** and Type **Clear Editor History** and [Enter] to clear VSCode cache files.

2. `$rustup update`

3. `$cargo clean`

4. Delete all the Rust relevant extensions

5. Intall only Rust[rls] package and test it again

You would have to wait until everything works fine before RLS build process end. Otherwise, restart your computer and [search more about this issue.](https://www.google.com/search?q=rls+building+not+working)

<br />

## 5. Conclusion

I hope you could make it and start to programm with Rust.

It may be difficult at the beginning. But you will find it becomes easier after your Rust code size grows up and have more projects in your store.

It may helpful for you to learn other programming languages such as C and operating system with it.

**Thanks and please share this post with others.**

(You may contact me if you need Rust or React JavaScript Developer for your work. I am also author of React [prop-passer] and [react-easy-md].)


