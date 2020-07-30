<!--
  Post {
    subtitle: "Learn how to set up Python development environment.",
    image: "post/Python/pygithub-example.jpg",
    image_decription: "Code by Steadylearner",
    tags: "How, dotenv, PyGithub, Python",
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

<!-- / -->

In this post, we will learn how to use [PyGithub] with dotenv.

It will help you before we learn how to build a [blog] with Python [Vibora] and GitHub in the later [Python blog posts].

You will have a Python file to get information from the GitHub repositories and your account. 

If you want the result first, please visit [blog] repository first.

<br />

<h2 class="red-white"> [Prerequistes] </h2>

1. [How to install Python]

2. [How to install pip]

3. [PyGitHub]

4. [dotenv]

5. [How to use IPython]

___

If you haven't used Python before. Please, read [How to install Python] and [How to install pip] to set up Python development environment first.

I want to you have made [GitHub account](https://github.com/) already and read [PyGitHub] and [dotenv] documenation first.

We won't directly need [IPython] here but learning [How to use IPython] will be very helpful.

<br />

<h2 class="blue">Table of Contents</h2>

1. GitHub Token to use its API

2. Build .env and settings.py to protect it

3. PyGitHub to get your account information at GitHub

---

<br />

## 1. GitHub Token to use its API

Before we use PyGitHub, we first need to get personal access token from the GitHub. [It won't be difficult if you follow the process from your search engine.](https://www.google.com/search?q=how+to+get+github+access+token)

You can also directly visit [https://github.com/settings/tokens] if you are already signed up with your GitHub account.

When you click the Generate New Token button at the top right of the view, you will have to submit a form with the **token descripition** and how you want to use it.

You may use **I will make Python GitHub blog** or else and select options you think necessary.

I hope you made it. Save it in your machine before you close the page.

<br />

## 2. Use dotenv to protect it

In the previous part, we made a personal access token to use GitHub API.

We should protect it first with [dotenv] not to allow others misuse it instead of you. Following the [dotenv] documentation, we will build **.env** and **settings.py**.

Install Python [dotenv] first if you haven't yet.

```console
$pip install -U python-dotenv
```

When it ends, set up **.env** with the token from the previous part.

```env
GITHUB_TOKEN=<GITHUB_TOKEN>
```

You can just paste the token you saved before without '' or "" in this file after **GITHUB_TOKEN=**.

Then, build **settings.py** with the token you made before.

```python
from dotenv import load_dotenv
from pathlib import Path
import os

env_path = Path('.') / '.env'
load_dotenv(dotenv_path=env_path)

GITHUB_TOKEN = os.getenv("GITHUB_TOKEN")

# print(f"My GitHub Token is {GITHUB_TOKEN} and I will build a blog with it.")
```

You can uncomment the **print** part above by deleting # and test it work with

```console
$python settings.py
```

and will show a message similar to this.

```console
My GitHub Token is <GITHUB_TOKEN> and I will build a blog with it.
```

It was simple and you should repeat the similar process whenever you want to protect some data from submitting to GitHub and others.

<br />

## 3.  PyGitHub to get your account information at GitHub

We already have everything we need to use PyGitHub API.

We will build **user.py** or **you.py** to get our GitHub account information with it.

The process is simple and we will use it to have a starting point to use [PyGitHub] API before we write Python CLI to handle [blog] posts and other files at [GitHub].

Install, [PyGitHub] first if you haven't yet.

```console
$pip install PyGithub
```

then use this to verify its API work or not.

```python
from settings import GITHUB_TOKEN # import GITHUB_TOKEN variable
from github import Github

g = Github(GITHUB_TOKEN)

you = g.get_user()

for your_repo in you.get_repos():
   print(your_repo.name)
```

when you execute it with

```console
$python user.py
```

It will show a message similar to this in your console.

```console
blog
Chat
code
pyscript
Python
Rust-Full-Stack
Rust-Jobs
Rust-Web-App
```

Then, imporve it and make it more meaningful for users.

First, install [termcolor] to decorate the console messages.

```console
$pip install termcolor
```

When it ends, edit the previous file while you refer to this.

```python
from settings import GITHUB_TOKEN # 1.
from github import Github
from termcolor import colored

g = Github(GITHUB_TOKEN)

you =  g.get_user()
repos = you.get_repos()
number_of_repos = repos.totalCount

colored_user_name = colored(you.name, attrs=['bold'])
colored_number_of_repos = colored(number_of_repos, "green")

print(f"\nYou({colored_user_name}) have {colored_number_of_repos} GitHub repositories.\n")

for num, repo in enumerate(repos, start = 1): # 2.
    print(
        colored(f"{str(num)}. {repo.name}", "blue") # 3.
    )

rust = g.get_repo("steadylearner/Rust-Full-Stack") # 4.

colored_rust_repo_star = colored(rust.stargazers_count, "yellow", attrs=['bold'])
colored_rust_repo_total_views = rust.get_views_traffic().get("count", "none")

print(f"\n{colored_rust_repo_star} stars in Rust-Full-Stack repository with {colored_rust_repo_total_views} views in total.")

```

It became a little bit more complciated than before. But, most of them are to use **colored** API from [termcolor] and not relevant to the logic of the programm. Read its documenation or remove the codes relevant to it if you think they are unncessary.

The payloads will be

**1.** Use **enumerate(repos, start = 1)** to use list of your **repositories** in **repo** variable and give numbers in front of them to make it more readable.

**2.** f in front of "" will help you format the string easily. You can just put Python variable you want to use inside {}.

**3.** You can use whatever GitHub user/repository name pair you want inside **get_repo()** method to get its information.

You may use yours or your favorite one then test it with

```console
$python user.py
```

and will show you the result simialr to this.

```console
You(Steadylearner) have 20 GitHub repositories.

1. blog
2. Chat
3. code
4. pyscript
5. Python
6. Rust-Full-Stack
7. Rust-Jobs
8. Rust-Web-App

34 stars in Rust-Full-Stack repository.
```

Include more mehtods while you inspect them with **dir(repos), help(repos)** with your console with **$python** or [IPython] and refer to [PyGithub] documentation.

<br />

## 4. Conclusion

In this post, we learnt how to use Python [dotenv] to protect your API token, [PyGithub] to use GitHub API and [termcolor] to decorate the console messages and make it more readable.

It will help you to learn how to build CLI to handle files saved at GitHub. Then, you can use it to build Python [Vibora] [blog] with the later [Python blog posts].

We will have many Python scripts similar to Rust files in [Rust Full Stack] with them.

If you want the latest contents, follow me at [Twitter] or [GitHub].

Do you need a **Full Stack Developer**? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others.**
