<!--
  Post {
    subtitle: "Learn how to make a thumbnail for all the images in a folder.",
    image: "post/Python/python-thumbnails-by-Steadylearner.png",
    image_decription: "Image by Steadylearner",
    tags: "How, Python, thumbnail, Pillow",
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

[How to make ssh]: https://www.digitalocean.com/docs/droplets/how-to/add-ssh-keys/create-with-openssh/

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack
[Rust blog posts]: https://www.steadylearner.com/blog/search/Rust

<!-- / -->

In this post, we will learn how to make thumbnails with Python [Pillow].

It already has a feature for that. Therefore, we just need to learn how to apply it in our project.

<br />

<h2 class="red-white"> [Prerequistes] </h2>

1. [Pillow]
2. [Python-Blog]

___

I want you already have experience with Python. If you haven't read [Pillow] documenation, please read it first. The code you will have is the part of [Python-Blog] project. You can refer to it if you want to find how to use it in a whole project.

<br />

<h2 class="blue">Table of Contents</h2>

1. Install Pillow
2. Make thumbnails with it
3. How to use it in frontend
4. Conclusion

---

<br />

## 1. Install Pillow

We will start by isntalling [Pillow] in our machines.

```console
$pip install pillow
```

It will install a few more relevant dependencies in your machine. Then, you can verify it was installed well with this.

```python
import PIL
print(PIL.PILLOW_VERSION)
```

It will show which version of Pillow you have in your machine on your console.

<br />

## 2. Make thumbnails with it

We could install [Pillow] to make thumbnail in the previous part. Then, we will write a **create_thumbanil** function to make thumbnails repeatedly.

Let me show you the code first.

```python
import os
from PIL import Image

def create_thumbnail(infile: str, include_in_path: str = "thumbnail", size: tuple = (256, 256)):
    infile_name, infile_extension = os.path.splitext(infile)
    outfile = f"{infile_name}.{include_in_path}{infile_extension}" # .png, jpeg etc
    if infile.find(include_in_path) == -1:
        if infile != outfile:
            try:
                im = Image.open(infile)
                im.thumbnail(size)
                im.save(outfile, infile_extension.split(".")[1].upper()) # PNG, JPEG etc
                print(f"Pillow made the thumbnail of {infile}.")
            except IOError:
                print(f"cannot create thumbnail for {infile}")
```

It has few lines of code but nothing complicated when you find what each parts do.

We have **infile**, **include_in_path** and **size** parameters for the function.

1. **infile** will be the name of the original image you want to make a thumbnail.

2. **include_in_path** will be used to differenciate the original file and the thumbnail file made from it.

3. **size** will be the width and height of the thbumnail image.

You may modify them.

Then, other parts are conditonals to not to make duplicate thumbnail images and a thubmnail image of a thumbnail image.

It won't be difficult to find what they do if you read the code line by line.

You can already use it for a single image. But, you can combine it with **os.walk** api and use it to build thumbnails for every images in a specific folder.

For example, write a Python file **make_thumbnails.py** similar to this.

```python
import os
from thumbnail import create_thumbnail

target = input("Which folder you will use to make thumbnails?(static/images by default)\n")

if target == "":
    target = "static/images"

list_of_file_paths = []

for folder_name, sub_folders, filenames in os.walk(target):
    for filename in filenames:
        entire_file_path = f"{folder_name}/{filename}"
        list_of_file_paths.append(entire_file_path)

print(list_of_file_paths)

for image_file_path in list_of_file_paths:
    create_thumbnail(image_file_path)
```

We won't learn deatils about this file. To make a thumbnail, we need a image file path and it just extract every file paths in specific folder and you can pass any function you want to use for them.

When you run this file with **$python make_thumbnails.py** and assign folders that includes your images.

You will see [Pillow] create thumbnails for the every images in the folder. If you haven't any project and images yet, you can verify it with [Python-Blog] and just use your images instead.

<br />

## 3. How to use it in frontend

There will be many reasons you want to create thumbnail images. But, what you want is to use thumbnail images instead of the original images in small devices, you can refer to the JavaScript code snippets below

**1.** With frameworks such as React

```js
const useThumbnail = (path = "") => {
  widthForThumbnail = "use the width you think adequate for this"
  if (window.innerWidth > widthForThumbnail) {
    return path;
  } else {
    const [file, extension] = path.split('.');
    const includeInPath = "thumbnail";
    const pathWithThumbnail = [file, includeInPath, extension].join('.');
    return pathWithThumbnail;
  }
}

// src = "local/image.png" from the state
<img src={useThumbnail(state.src)} />
```

**2.** Pure JavaScript

```js
<img id="responsive-image" src="local/image.png" />

widthForThumbnail = "use the width you think adequate for this"
responsiveImage = document.getElementById("responsive-image")
path = responsiveImage.src

const useThumbnail = (path = "") => {
  if window.innerWidth > widthForThumbnail {
    responsiveImage.src = path
  } else {
    const [file, extension] = path.split('.');
    const includeInPath = "thumbnail";
    const pathWithThumbnail = [file, includeInPath, extension].join('.');
    responsiveImage.src = pathWithThumbnail;
  }
}
```

It will help your client to use thumbnail images for src in small devices when it is below the size you define with **widthForThumbnail**.

<br />

## 4. Conclusion

In this post, we learnt how to use Python [Pillow] to make a thumbnail and apply it to a folder to create thumbnail images for the every images in it.

We will have many Python scripts and blog posts to explain them. If you just want the result, please use files in [Python-Blog] and [automation] repository. For frontend part, [Rust Full Stack] and [Rust blog posts] will be also useful.

If you want the latest contents, follow me at [Twitter] and [GitHub].

Do you need a **Full Stack Developer**? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others.**
