<!--
  Post {
    subtitle: "Learn how to set up Scrapy development environment and use it",
    image: "post/Python/scrapy.png",
    image_decription: "Image from the official website,
    tags: "How, Python, Scrapy, use",
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

[Pillow]: https://pillow.readthedocs.io/en/stable/
[Pillow Installation]: https://pillow.readthedocs.io/en/stable/installation.html

[Scrapy]: https://scrapy.org/
[Scrapy Tutorial]: https://docs.scrapy.org/en/latest/intro/tutorial.html
[Scrapy CLI]: https://docs.scrapy.org/en/latest/topics/commands.html
[scrapy-examples]: https://github.com/steadylearner/scrapy-examples
[How to use selectors with Scrapy]: https://docs.scrapy.org/en/latest/topics/selectors.html

[How to make ssh]: https://www.digitalocean.com/docs/droplets/how-to/add-ssh-keys/create-with-openssh/

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack
[Rust blog posts]: https://www.steadylearner.com/blog/search/Rust

[Rust notification website]: https://this-week-in-rust.org

[Steadylearner]: https://www.steadylearner.com

<!-- / -->

In this post, we will learn how to use Python [Scrapy]. We will use [Rust notification website] **This Week In Rust** as an example. If you are a [Rust developer][Rust Full Stack], You will find you can easily extract only the parts you want from its pages.

Otherwise, use whatever website you want and refer this blog post briefly.

<br />

<h2 class="red-white"> [Prerequistes] </h2>

1. [How to install Python]
2. [How to install pip]
3. [Scrapy]

___

I want you already have experience with Python. It will be helpful for you to spend a few hours to read [Scrapy] documentations.

<br />

<h2 class="blue">Table of Contents</h2>

1. Setup development environment
2. Inspect the website
3. Write Python Scrapy code
4. Conclusion

---

You can skip **1.** if you already have Scrapy development environment ready. If you just want the paylaods of Scrapy projects, please refer to the [scrapy-examples] repository.

<br />

## 1. Setup development environment

We will start by setting Scrapy development environment with pip. Use this command.

```console
$python3 -m venv scrapy
```

It will make a structure similar to this in your machine with directory name **scrapy**.

```console
bin  include  lib  lib64  pyvenv.cfg  share
```

We don't have to care for others and our interest will be only **bin/activate** file to use virutalenv. We should activate Python development environment to use it.

You can manually do that everytime but making alias for it will be better. Use this command.

```console
$vim ~/.bashrc
```

Then, include the code similar to this.

```console
alias usescrapy="source /home/youraccount/Desktop/code/scrapy/bin/activate"
```

You should find the equivalent part of **/home/youraccount/Desktop/code/** with **$pwd** command if you want to use this. Then, **$source ~/.bashrc** and you can use this Python development environment with **$usescrapy** only whenever you want.

Type **$usescrapy** and **$pip install ipython scrapy**. It will install the minimal dependencies to use Python Scrapy. If you like the Python dependencies and reuse the exactly same ones later, use these commands.

1. **$pip freeze > requirements.txt** to extract the list of them.
2. **$pip install -r requirements.txt** to install them later.

<br />

## 2. Inspect the website

I hope you already visited the [Rust notification website] or other websites you want to crawl with Python [Scrapy]. This will be a personal choice. Refer to the processes used here briefly with [Scrapy Tutorial] and apply it to your target websites.

I will assume you have used [browser inspector](https://www.google.com/search?q=how+to+use+browser+inspector) and familar with CSS and HTML.

The purpose of This Week In Rust is to give you useful links relevant to **Rust** for each week.

It has the recent 5 publications links in the homepage. 

When you visit each of them, you will see the list of links for blog posts, crates(packages in Rust), call for particpation, events, jobs etc.

Back to its homepage and use your browser inspector with **CRTL+SHIFT+I** and find how its html is structured. You can see that it is just simple static website with a CSS framework.

Inspect **This week in Rust $Number of publication** part. Then, you will find many **a html tags** similar to this.

```html
<a href="https://this-week-in-rust.org/blog/this-week-in-rust/">This Week in Rust</a>
```

Collecting those links to follow will be our main job for this page. We will write Python Scrapy code for it later.

Visit one of them. When you inspect posts, jobs parts and other parts you want to scrap, you will see that they have similar strcuture similar to this.

```html
<h2 id="news-blog-posts"></h2>
<ul>
  <li>
    <a href="payload" ></a>
  </li>
</ul>
```

Our main target will be **href** to help you get to the useful blog posts and job vacancies. It is the part of **a** tag that are wrapped with **li** and its parent element **ul**. 

You can see that **ul** is also followed by **h1** or **h2** tags with ids. This will facilitate finding those **href** parts with Scrapy later.

<br />

## 3. Write Python Scrapy code

We set up development environmenet and have the information ready to use with the previous parts. What left is to write the Python code for Scrapy to work.

Before that, use shell command from [Scrapy CLI] to test how the Scrapy programm will see the webpage.

```console
$scrapy shell https://this-week-in-rust.org
```

Use your target websites instead if you want. Then, the console will become the Ipython mode with information similar to this.

```console
[s] Available Scrapy objects:
[s]   scrapy     scrapy module (contains scrapy.Request, scrapy.Selector, etc)
[s]   crawler    <scrapy.crawler.Crawler object at>
[s]   item       {}
[s]   request    <GET https://this-week-in-rust.org>
[s]   response   <200 https://this-week-in-rust.org>
[s]   settings   <scrapy.settings.Settings>
[s]   spider     <DefaultSpider 'default'>
[s] Useful shortcuts:
[s]   fetch(url[, redirect=True]) Fetch URL and update local objects (by default, redirects are followed)
[s]   fetch(req)                  Fetch a scrapy.Request and update local objects
[s]   shelp()           Shell help (print this help)
[s]   view(response)    View response in a browser
```

Use **$view(response)** first to verify your target websites can be read by Scrapy. For example, if the website is rendered with JavaScript, it may not work well and you should [find more documentation][Scrapy] for that.

Test it with [Steadylearner] built with React frontend or other single page app websites. With This Week In Rust, there was no problem because it is just a normal static website.

You can play with Scrapy shell mode with request, response etc. For example, use **response.body**, **response.title**. Then, exit it and start your Scrapy projects with command similar to this.

Use **$scrapy startproject notification rust**. It will automatically generate scrapy project folder with **rust** and project name **notification** and will show message similar to this in your console.

```console
    cd rust
    scrapy genspider example example.com
```

You can use **$scrapy startproject -h** for more information.

Follow the instruction and use command similar to **$scrapy genspider this_week_in_rust this-week-in-rust.org/**.

It should have created **spiders/this_week_in_rust.py** in your machine. You can see that Python **Scrapy** framework does most of jobs instead of you.

The prepartion process ends. We will write code for the spider(this_week_in_rust.py). Edit it similar to this.

```python
# this_week_in_rust.py
# -*- coding: utf-8 -*-
import scrapy

class RustNotificationSpider(scrapy.Spider):
    name = 'this_week_in_rust'
    start_urls = ['https://this-week-in-rust.org/']

    # 1.
    def parse(self, response):

        for href in response.css("div.custom-xs-text-left > a::attr(href)").getall():
            yield response.follow(href, self.parse_post_and_jobs)

    # 2.
    def parse_post_and_jobs(self, response):
        date = ".".join(response.url.split("/")[4:7]).replace(".","-")

        post_titles = response.css("#news-blog-posts + ul > li > a::text").getall()
        post_urls = response.css("#news-blog-posts + ul > li > a::attr(href)").getall()
        posts = { "posts": len(post_titles), **dict(zip(post_titles, post_urls)) }

        job_titles = response.css("#rust-jobs + ul > li > a::text").getall()
        job_urls = response.css("#rust-jobs + ul > li > a::attr(href)").getall()
        jobs = { "job": len(job_titles), **dict(zip(job_titles, job_urls)) }

        # sorted(list, key = lambda i: i["Posts"], reverse = True)
        yield {
            "date": date,
            **posts,
            **jobs,
        }
```

Nothing complciated here and we just convert the information we get from the part **2.** into Python Scrapy code.

**1.** we extract the publication page links to follow with [CSS Selectors][How to use selectors with Scrapy]. **div.custom-xs-text-left** is to help select href part in a tags better. We had 5 links to follow so use getall().

Then, we define what to do with them with **parse_post_and_jobs** callback function.

**2.** This is payload of all these processes. We extract date of the publication, the total number of them, titles and other important datas of Rust blog posts and jobs to make the information more userful. Then, we turn it into JSON format with Python API.

You can see the pattern that only id part such as **#news-blog-posts, #rust-jobs** are diefferent and others are repeated. You can easily include eventy, call for participation etc from the website if you want to scrap other parts.

Your code will be different from this if you used other websites but the process will be similar.

1. Get the links to follow to visit the payload webpages.
2. Extract the payload information you want.

Test it work with **$scrapy crawl this_week_in_rust -o summary.json**. Then, you can verify the result similar to this structure.

```json
[
    {
        "date"
        "posts"
        "job"
    },
]
````

It may not be ordered well by date. Therefore, make Python file similar to this if you want.

```python
# sort_json.py
import os
import sys

import json

target = sys.argv[1]

with open(target, 'r') as reader:
    json_str = reader.read()
    json_lists = json.loads(json_str) # dict, read

    with open(target, 'w+') as writer:
        sorted_list = sorted(json_lists, key = lambda i: i["date"], reverse = True) # only work for list of dicts
        json_sorted_str = json.dumps(sorted_list, indent=4) # write
        writer.write(json_sorted_str)

print(f"Sort {target} by dates.")
```

Use it with **$python sort_json.py summary.json** and it will organize the JSON file by dates.

You should **comment** or remove **sort_json.py** from your Scrapy project if you want to use this project later.

<br />

## 4. Conclusion

In this post, we learnt how to use Python [Scrapy]. If you followed this post well, what you need later will be just use **$scrapy genspider** and edit the Python file(spider) made from it.

We will have more Python Scrapy spiders and blog posts for them. If you just want more examples from [Steadylearner], please refer to the [scrapy-examples] repository.

I hope this post be helpful for other Rust developers who wait for **This Week In Rust** every week and also for people who want to learn Python Scrapy.

I am a full stack developer and use [Rust][Rust Full Stack], **Python**, **JavaScript** etc. I can also help you with **Docker** and **AWS** etc to deploy your project.

If you want the latest contents, follow me at [Twitter] and [GitHub].

Do you need a **Full Stack Developer**? Conatct me with [LinkedIn].

**Thanks and please share this post with others.**
