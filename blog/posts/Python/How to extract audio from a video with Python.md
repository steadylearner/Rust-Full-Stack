<!--
  Post{
    subtitle: "Save audio files from videos with Python Moviepy",
    image: "/brand/Moviepy.png",
    image_decription: "Image from its website",
    tags: "Python, How, video, audio",
  }
-->

<!-- Link -->

[YouTube]: https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw
[ImageMagik]: https://www.steadylearner.com/static/images/brand/ImageMagik.gif
[Moviepy]: https://zulko.github.io/moviepy/
[sys-env in Python]: https://stackoverflow.com/questions/4906977/how-do-i-access-environment-variables-from-python

<!-- / -->

In this post, we will learn how to extract only audio files from videos with Python.

Before finding working **Python** code for the purpose of this post, I thought it would be easy to find working examples. But, it was not and that is the reason I write this post.

If I do not know how to use Python or other programming languages. I would repeatedly separate audio files, opening videos and wating for software to separate audio clip and video clip for that.

Then, I should compile only audio files and wait for a long time unnecessarily.

But I would not do that. With the power of programming, we do not have to repeat what we already know.

<h2 class="red-white"> [Prerequistes] </h2>

1. [How to install Python](https://realpython.com/installing-python/)

2. [How to install pip](https://linuxize.com/post/how-to-install-pip-on-ubuntu-18.04/)

3. [Moviepy official website](https://zulko.github.io/moviepy/install.html)

4. [sys-env in Python]

___

You should Python and pip have installed in your machine. Python would be already in your machine.

Type `$python` in your console to verify that.

The **pip** will be used to install and use [Moviepy]. It will help you to extract audio files from videos with few lines of code. 

You can install it with `$pip install moviepy`.

<br />

<h2 class="blue">Table of Contents</h2>

1. Python Code to extract audios from the videos

2. **Conclusion**

---

<br />

## 1. Python Code to extract audios from the videos

I hope you already prepared a video file with audio for this process. The process is very simple so let me show you the code first.

```python
# extract_audio.py

# 1.
import sys
from moviepy.editor import *

video = VideoFileClip(sys.argv[1]) # 2.
audio = video.audio # 3.
audio.write_audiofile(sys.argv[2]) # 4.
```

You can save this file with **extract_audio.py** and test it with

```console
$python extract_audio.py <video_file> <audio_file>
ex) $python extract_audio.py video_for_audio.mp4 audio_from_video.mp4
```

It will make audio_from_video.mp4 file in your machine.

If I explain it with details, what we do are

1. We import default Python **sys** module to use **sys.argv** later and **moviepy.editor** to extract audio files from videos.

2. In **video = VideoFileClip(sys.argv[1])** you instruct your machine which video file you want to use to extract audio.

3. Then, with **audio = video.audio**, you select audio file inside the video file. You can see that Python use **.** syntax for that.

4. Lastly, with **audio.write_audiofile(sys.argv[2])**, You end the process and assign the result audio file name.

and that is all process you need to take to extract audio from the video with python.

If you know nothing about Python or other programming, maybe you are confused what does sys or sys.argv mean. They help you automate your work with CLI.

What comes first after **$python file.py** would be **sys.argv[1]** and others would be sys.argv[2] and more.

<br />

## 2. Conclusion

<br />

<section class="width-percent flex center">
  <a title="Link to Steadylearner YouTube Channel" alt="Link to Steadylearner YouTube Channel" href="https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw"><img width="80px" class="flex center hover cursor-pointer" src="https://www.steadylearner.com/static/images/brand/Steadylearner_YouTube.png" /></a>
</section>

<br />

I want you learnt that you donât need software to edit videos and audios for the simple process.

You can find some of the example videos at [Steadylearner YouTube][YouTube] if you haven't any to test.

**Thanks and please share this post with others.**
