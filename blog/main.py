# Use __init__.py for the file name (if you want to publish the package.)

from vibora import Vibora, Response
from vibora.static import StaticHandler

from settings import GITHUB_TOKEN
from github import Github

import requests

from lib import remove_meta

app = Vibora(
    static=StaticHandler(
        paths=['static'],
        url_prefix='/static',
    ),
    template_dirs=['templates'],
)

REPO_NAME = "Python-Blog"

@app.route('/')
async def home():
    payload = "https://raw.githubusercontent.com/steadylearner/Python-Blog/master/README.md"

    # https://www.google.com/search?q=what+is+"Cache-Control
    # To refresh the page whenever you update README.md and verify the result
    # Comment it in production
    headers = {
        "Cache-Control": "no-cache",
        "Pragma": "no-cache"
    }

    content = requests.get(
        payload,
        headers=headers
    ).text

    print(content)

    content_without_meta = remove_meta(content) # or content only if you don't use meta data

    return await app.render('home.html', content=content_without_meta)

# @app.route('/')
# async def home():
#     return Response(b"Hello, World")

# @app.route('/blog')
# async def blog():
#     return Response(b"Show the list of blog posts with the templates and GitHub")

@app.route('/blog/<lang>')
async def blog(lang: str):
    g = Github(GITHUB_TOKEN)

    repo = g.get_repo(f"steadylearner/{REPO_NAME}")
    filepath = f"posts/{lang}" # posts/Rust

    contents = repo.get_contents(filepath)

    posts = []
    while contents:
        file_content = contents.pop(0)
        if file_content.type == "dir":
            contents.extend(repo.get_contents(file_content.path))
        else:
            payload = file_content.path
            title = payload.split("/")[-1].split(".")[0]
            posts.append(title)

    print(posts)

    return await app.render('list.html',
        lang=lang,
        posts=posts,
    )

@app.route("/blog/<lang>/<title>")
async def show_post(lang: str, title: str):
    # Test with

    # http://0.0.0.0:8000/blog/Rust/README
    # http://0.0.0.0:8000/blog/Rust/How-to-render-blog-posts-with-Rust-Yew-mounted-API
    # http://0.0.0.0:8000/blog/Rust/How%20to%20render%20blog%20posts%20with%20Rust%20Yew%20mounted%20API
    # (http://0.0.0.0:8000/blog/Rust/How to render blog posts with Rust Yew mounted API)

    # or visit /blog/Rust

    title = title.replace("-", "%20")

    # https://raw.githubusercontent.com/steadylearner/blog/master/posts/Rust/How%20to%20render%20blog%20posts%20with%20Rust%20Yew%20mounted%20API.md

    prefix = f"https://raw.githubusercontent.com/steadylearner/{REPO_NAME}/master/posts"
    payload = f"{prefix}/{lang}/{title}.md"
    # print(payload)

    content = requests.get(payload).text
    content_without_meta = remove_meta(content) # or content only if you don't use meta data

    post = {
        "title": title.replace("%20", " "),
        "content": f"{content_without_meta}",
    }

    # post = {
    #     "title": title.replace("%20", " "),
    #     "content": f"{content}",
    # }

    return await app.render('post.html',
        title=post["title"],
        content=post["content"],
    )

# @app.route("/blog/<title>")
# async def show_post(title: str):
#     return Response(f"Chosen blog: {title}".encode())

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=8000)


