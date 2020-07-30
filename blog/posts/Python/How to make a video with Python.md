<!--
  Post{
    subtitle: "Upload your video made with Python at YouTube",
    image: "/brand/youtube-brand-image.jpg",
    image_decription: "YouTube Image",
    tags: "Python, How, video, code",
  }
-->

<!-- Link -->

[YouTube]: https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw
[ImageMagik]: https://www.steadylearner.com/static/images/brand/ImageMagik.gif
[Moviepy]: https://zulko.github.io/moviepy/

<!-- / -->

In this post, You will learn how to make a music video with a single image. You should have seen many music videos like that at [YouTube]. You will find that you don't need complicated softwares to make them.

What you need is just **Python, audio and images files** to use. Before move on, I hope you already know how to programming a little bit and have Python and pip installed in your machine.

If it wasn't, that is ok. You may read links below for that.

<br />

<h2 class="red-white"> [Prerequistes] </h2>

1. [How to install Python](https://realpython.com/installing-python/)

2. [How to install pip](https://linuxize.com/post/how-to-install-pip-on-ubuntu-18.04/)

3. [Moviepy official website](https://zulko.github.io/moviepy/install.html)

4. [How to install ImageMagik](https://imagemagick.org/script/install-source.php)

5. [How to use youtube-dl](https://github.com/ytdl-org/youtube-dl)

6. [How to extract audio from the video with Python](https://medium.com/@steadylearner/how-to-extract-audio-from-the-video-with-python-aea325f434b6)

___

You wouldn't need **ImageMagick** and **youtube-dl** for this post if you already have audio and image files ready. You can skip the parts relevant to them if you want. I also included **How to extract audio from the video with Python** for it will help you understand this post better.

From now on, I will assume that you already made Python and pip work for your machine. Most of the time, Python will be already installed in your local machine. I hope you to use the lastest version of it to stay relevant to this posts.

Some Linux distributions such as Ubuntu 18.04 already include ImageMagik. You may test it with `$convert` command and follow the instruction and read the [manual][How to install ImageMagik].

<br />

<h2 class="blue">Table of Contents</h2>

1. Prepare audio with **youtube-dl**

2. Edit image size with **ImageMagik**

3. **Python** code with **Moviepy** to make a video

4. **Conclusion**

---

<br />

## 1. Prepare audio with youtube-dl

<a title="Link to YouTube Video for this post" class="hover cursor-pointer" href="http://www.youtube.com/watch?feature=player_embedded&amp;amp;v=s7TVVyTyReU
" target="_blank"><img src="http://img.youtube.com/vi/s7TVVyTyReU/0.jpg"
alt="Your Love - The Outfield" width="100%" height="100%" border="10" /></a>

I hope you already made **youtube-dl** installed following the official documentation from it. We will use it to donwload YouTube and then extract it with [Moviepy] later.

You can search videos at YouTube for the audio. Then you can use its id instead of `<videId>` below to donwload them.

```console
$youtube-dl https://www.youtube.com/watch?v=<videoId>

ex) $youtube-dl https://www.youtube.com/watch?v=s7TVVyTyReU
```


<br />

## 2. Edit image size with ImageMagik

![ImageMagik]

For audio file is ready with the [YouTube Video](https://www.youtube.com/watch?v=s7TVVyTyReU) from the previous prcoess, we will prepare the image that will be used with it. I hope you already have any software for image edition unless you want to use ImageMagik or couldn't make it installed.

In this phase, We will make 640x360 image with ImageMagik for it works well with YouTube and other cases also.

You can type command below for that.

```console
$convert -resize 640x360! your_image.ext result_image.ext 

ex) $convert -resize 640x360! image_from_the_web.jpeg resized_image.jpeg
```

and that is all.

Your image is ready to be used for your music video with a single image.
(Use ! to force image resize, otherwise it will find the size to preserve image quality.)

<br />

## 3.  Python code with Moviepy to make a music video

<br />

<section class="width-percent flex center">
  <section><img width="100%" class="flex center hover cursor-pointer" src="https://www.steadylearner.com/static/images/brand/Moviepy.png" /></section>
</section>

<br />

At this moment, You already have audio file and image file to use. Just instruct your machine to make a video instead of you with Python. We will Moviepy for that.

**I hope you already visited its official website and read the documenation.**

The code snippet we need to write will be just a few lines of code. Then, execute it at command line.

Let me show you the code first.

**[Python Code]**

```python
# music_video_with_image.py
# 1.
import os, sys
from moviepy.editor import *

# 2.
audio = AudioFileClip(sys.argv[1])
image = ImageClip(sys.argv[2]).set_duration(audio.duration)

video = image.set_audio(audio)
outfile = f"{os.path.splitext(sys.argv[1])[0]}_with_image.mp4" # 3.

video.write_videofile(outfile, fps=1)

# Use line below if you want to preserve the name of both files.
# 1. outfile = f"{os.path.splitext(sys.argv[1])[0]}_with_image_{os.path.splitext(sys.argv[2])[0]}.mp4"
```

<br />

1. Import os, sys to handle file names for our preference and to use CLI with arguments. Then, `from moviepy.editor import *` to use every functions of Moviepy inside this file.

2. Write Python code on our own. They are similar to plain english. You can see that `AudioFileClip` only extract audio file from the video and you can use it for image files made from `ImageClip`.

3. Use variables for each files that we prepared above. Then, use Moviepy and Python native API to make a file with name **f"{your_preferred_video_filename_and.ext}**.

<br />

You may want to know what do sys.argv[1] and sys.argv[2] mean.

They are just used to indicate arguments that you will pass in your console later.

You may visit [How to use sys.argv in Python](https://www.google.com/url?sa=t&amp;amp;rct=j&amp;amp;q=&amp;amp;esrc=s&amp;amp;source=web&amp;amp;cd=3&amp;amp;cad=rja&amp;amp;uact=8&amp;amp;ved=2ahUKEwiF7sPxmY7hAhV1ILkGHcHODyIQFjACegQIABAL&amp;amp;url=https%3A%2F%2Fwww.pythonforbeginners.com%2Fsystem%2Fpython-sys-argv&amp;amp;usg=AOvVaw0R5P3WpDvi-MMdfy1q1YQQ) for more information.

You may save it with file name such as **music_video_with_image.py** for the next phase.

<br />

**[Command Line]**

```console
$python music_video_with_single_image.py <video_for_audio> <image>

ex) `$python music_video_with_image.py audio_from_video.mp4 your_image.jpg`
```

You almost made it. Type the similar command in your console with different file names. Then, You will see the process similar to snippet below and your video file will be ready to use.

(You can see the video made with this process at [here](https://www.steadylearner.com/video/watch/s7TVVyTyReU))

```console
$python image_with_audio.py The\ Outfields\ -\ I\ don\'t\ wanna\ lose\ your\ love\ tonight.webm outfiled-yourlove.jpeg
pygame 1.9.4
[MoviePy] >>>> Building video The Outfields - I don't wanna lose your love tonight_with_image_outfiled-yourlove.mp4
[MoviePy] Writing audio in The Outfields - I don't wanna lose your love tonight_with_image_outfiled-yourloveTEMP_MPY_wvf_snd.mp3
[MoviePy] Done.
[MoviePy] Writing video The Outfields - I don't wanna lose your love tonight_with_image_outfiled-yourlove.mp4
[MoviePy] Done.
```

<br />

## 4. Conclusion

<br />

<section class="width-percent flex center">
  <a title="Link to Steadylearner YouTube Channel" alt="Link to Steadylearner YouTube Channel" href="https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw"><img width="80px" class="flex center hover cursor-pointer" src="https://www.steadylearner.com/static/images/brand/Steadylearner_YouTube.png" /></a>
</section>

<br />

I hope you made it. Just modify some words in examples for your image and video. You may upload the file at YouTube or use it to learn Python.

You can also find some of the example videos at [Steadylearner YouTube][YouTube]. They are made with almost the same process explained here.

**Thanks and please share this post with others.**
