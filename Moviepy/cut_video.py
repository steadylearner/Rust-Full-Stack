# Refer to 

import sys
from moviepy.editor import *

# print(f"{sys.argv}\n")

original = sys.argv[1] # demo.mp4
result = sys.argv[2] # short.mp4
start = sys.argv[3] # 0
end = sys.argv[4] # 60

# $python cut_video.py demo.mp4 cut.mp4 0 60

sub_clip = VideoFileClip(original).subclip(start, end)

new_video = CompositeVideoClip([sub_clip])
new_video.write_videofile(result)


