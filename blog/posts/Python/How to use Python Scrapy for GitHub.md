<!--
  Post {
    subtitle: "Learn how to use Scrpay to crawl GitHub issues.",
    image: "code/GitHub.png",
    image_decription: "Image from the official website,
    tags: "How, GitHub, Scrapy, use",
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

[GitHub Vaga]: https://github.com/frontendbr/vagas
[GitHub Job Issues]: https://github.com/frontendbr/vagas/issues

<!-- / -->

<!-- Steadylearner Posts -->

[How to use Python Scrapy to crawl static websites]: https://www.steadylearner.com/blog/read/How-to-use-Python-Scrapy-to-crawl-static-websites
[Rust Jobs]: https://github.com/steadylearner/Rust-Jobs

<!-- / -->

In the previous post about scrapy, we learnt how to use Python [Scrapy] with [Rust notification website] **This Week In Rust**. In this post, we will see how to aplly what we learnt to [GitHub].

We will use [GitHub Frontend Job Openings and others][GitHub Vaga] used to procuer developers by Brazilian Portuguish users. It is not written in Enlgish. But, I made a Rust version at [Rust Jobs] and wouldn't be difficult to find what they do.

For some of them are compartively active and updated by real people, you can use this as an opportunity to learn [Scrapy] for the dynamic data.

Many parts of this post will be duplicate to the previous post [How to use Python Scrapy to crawl static websites].

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

I hope you already visited [the website][GitHub Vaga]. Then, you will find it has [GitHub Job Issues] link.

Recuriters use this to make a GitHub issue to find the developers they want. They use **[City]Title** as a title of issue. When you visit it you will find some details about a job.

Most of the times, it is about the company and requirements for the candidates with email to receive resumes. The location and Email will be the payload of this. We will use [Scrapy] to learn how to extract those datas.

For that, we first need to inspect HTML and CSS of these pages. Then, we will convert the information into Python codes later.

I will assume you have used [browser inspector](https://www.google.com/search?q=how+to+use+browser+inspector) before and familar with CSS and HTML.

When you visit [GitHub Job Issues], you will see the list of links for job openings. Use your browser inspector with **CRTL+SHIFT+I** and find how its html is structured.

You can see that most of them are very similar. In this list, **title text** and **href** of each issue are payloads.

Inspect one of those links. You will find we can use **data-hovercard-type** to extract href and title text later.

```html
  <a data-hovercard-type="issue" href="payload" >[City] Title</a>
```

Visit one of them. When you inspect an issue, you will find it has 


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
# -*- coding: utf-8 -*-
import scrapy

class RemoteJobSpider(scrapy.Spider):
    name = 'remote_job'
    start_urls = [
        "https://github.com/frontendbr/vagas/issues",
        "https://github.com/react-brasil/vagas/issues",
        "https://github.com/backend-br/vagas/issues",
    ]

    def parse(self, response):

        # text, href dict
        # filter it with text
        # use href

        titles = response.xpath("//a[@data-hovercard-type='issue']/text()").getall()
        hrefs = response.xpath("//a[@data-hovercard-type='issue']/@href").getall()

        before_filter = dict(list(zip(titles, hrefs)))

        # remote_variations = ["Remote", "remote", "Remoto", "remoto"]

        with_remote = dict(filter(lambda elem:
            "Remote" in elem[0] or "Remoto" in elem[0] or "remoto" in elem[0] or "remote" in elem[0]
        , before_filter.items()))

        print(with_remote)

        for item in with_remote.items():
            href = item[1]
            # preifx = "https://github.com/react-brasil/vagas/issues"
            yield response.follow(href, self.parse_jobs)

    def parse_jobs(self, response):
        print(response.url)

        title = response.xpath("//span[@class='js-issue-title']/text()").get().strip()
        urls = response.css("tbody  a::attr(href)").getall()
        email_list = []

        for payload in urls:
            if payload.startswith("mailto"):
                email_index = urls.index(payload)
                email_with_mailto = urls.pop(email_index)
                email = email_with_mailto.split(":")[1]
                email_list.append(email)

        yield {
            "title": title,
            "urls": urls,
            "email_list": email_list,
            # "page": response.url
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
