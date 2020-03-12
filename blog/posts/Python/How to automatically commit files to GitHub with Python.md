<!--
  Post {
    subtitle: "Use Python subprocess to automate the commit to GitHub.",
    image: "post/Python/python-github.jpg",
    image_decription: "Code by Steadylearner",
    tags: "How, commit, Python, GitHub",
  }
-->

<!-- Link -->

[PyGithub]: https://github.com/PyGithub/PyGithub
[dotenv]: https://github.com/theskumar/python-dotenv
[Vibora]: https://github.com/vibora-io/vibora
[termcolor]: https://pypi.org/project/termcolor/
[IPython]: https://github.com/ipython/ipython

[blog]: https://github.com/steadylearner/blog

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack

[Python blog posts]: https://www.steadylearner.com/blog/search/Python

[Twitter]: https://twitter.com/steadylearner_p
[GitHub]: https://github.com/steadylearner
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

[How to install Python]: https://realpython.com/installing-python/
[How to install pip]: https://linuxize.com/post/how-to-install-pip-on-ubuntu-18.04/

[How to use IPython]: https://www.google.com/search?q=how+to+use+ipython

[Python-Blog]: https://github.com/steadylearner/Python-Blog
[automation]: https://github.com/steadylearner/automation

[How to make ssh]: https://www.digitalocean.com/docs/droplets/how-to/add-ssh-keys/create-with-openssh/

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack
[Rust blog posts]: https://www.steadylearner.com/blog/search/Rust

<!-- / -->

In this post, we will learn how to save our time with Python **subprocess**. We will make simple script file with it instead of repeating the same GitHub CLI.

What you will need at the end of this post will be just type

```console
$python commit.py
```

instead of

```console
$git add .
$git commit -m "your commit message"
$git push -u origin master

and type your GitHub id and password
```

<br />

<h2 class="red-white"> [Prerequistes] </h2>

1. [How to install Python]

2. [How to install pip]

___

If you haven't used Python before. Please, read [How to install Python] and [How to install pip] to set up Python development environment first.

I want to you have made [GitHub account](https://github.com/) already and experience with it.

<br />

<h2 class="blue">Table of Contents</h2>

1. How to use subprocess
2. Build commit.py with it to automate the process
3. How to skip typing id and password with ssh

---

<br />

## 1. How to use subprocess

Before we build commit.py script, we should test Python subprocess module first. It is just to help you use built in POSIX commands such as **echo**, **ls** etc.

You can test it with $python or [IPython].

```python
import subprocess as cmd
cmd.run("echo 'I will automate everythig with Python.'", check=True, shell=True)
```

and it will be no different from

```console
$echo 'I will automate everythig with Python.'
```

You will want to use **check=True, shell=True** part normally. Then, substitute **echo 'I will automate everythig with Python.'** with the commands you want to use.

Nothing complicated here. If you want more information, you can use **help(subprocess)**, **dir(subprocess)** or search in your browser **"How to use Python subprocess"**.

<br />

## 2. Build commit.py with it to automate the process

We already learnt how to use subprocess in the previous part. In this part, we will bring the working example from [Python-Blog] and learn how to use it.

The commit.py in the repository will be similar to this.

```python
import subprocess as cmd

cp = cmd.run("git add .", check=True, shell=True)
#print(cp)

response = input("Do you want to use the default message for this commit?([y]/n)\n")
message = "update the repository"

if response.startswith('n'):
    message = input("What message you want?\n")

cp = cmd.run(f"git commit -m '{message}'", check=True, shell=True)
cp = cmd.run("git push -u origin master -f", check=True, shell=True)
```

It has just a few lines of code.

You just need to put GitHub CLI commands you want to automate inside Python **subprocess** API we learnt before.

We could also cutomize our commit mesage with

```python
response = input("Do you want to use the default message for this commit?([y]/n)\n")
message = "update the repository"
```

You could also inspect each process with **print(cp)**.

Verify it work with **$python commit.py**. It will be much faster than manually repeat every commands. But, you need to type GitHub id and password yet.

<br />

## 3.  How to skip typing id and password with ssh

We are almost there to automate the whole process to commit files to GitHub.

What left is to find [how to skip login process for github push](https://www.google.com/search?q=how+to+skip+login+process+for+github+push). You can search on your own and read some blog posts before read on.

There are various ways for that but we will use **ssh** in this post. If you don't have it yet, please read [How to make ssh] first. Otherwise, [it may be in your machine already](https://help.github.com/en/articles/checking-for-existing-ssh-keys). Please, use it then.

When you are with ssh in your machine, you can register it to GitHub with

1. https://help.github.com/en/articles/connecting-to-github-with-ssh
2. https://github.com/settings/keys

Then, you can start your GitHub repository with ssh

```console
$git remote add origin git@github.com:steadylearner/Python-Blog.git
```

or substitute the existing https connection with

```console
$git remote set-url origin git@github.com:steadylearner/Python-Blog.git
```

to commit your files to the repository only **$python commit.py** instead of

```console
$git add .
$git commit -m "your commit message"
$git push -u origin master

and the login process.
```

<br />

## 4. Conclusion

In this post, we learnt how to use Python subprocess to automate github commit with ssh. The set up process is a little bit tedious but the reward of it will be your saved time.

It will help you to commit your files instantly to GitHub. Refer to commit.py in [Python-Blog] and modify it for your project.

We will have many Python scripts and blog posts to explain them. If you just want the result, please use files in [Python-Blog] and [automation] repository. For frontend part, [Rust Full Stack] and [Rust blog posts] will be also useful.

If you want the latest contents, follow me at [Twitter] and [GitHub].

Do you need a **Full Stack Developer**? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others.**
