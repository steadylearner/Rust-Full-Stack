import os
# from termcolor import colored

from thumbnail import create_thumbnail

target = input("Which folder you will use to make thumbnails?(static/images by default)\n")

if target == "":
    target = "static/images"

list_of_file_paths = []

# refer to traverse_dir.py in this folder
for folder_name, sub_folders, filenames in os.walk(target):
    for filename in filenames:
        entire_file_path = f"{folder_name}/{filename}"
        list_of_file_paths.append(entire_file_path)

print(list_of_file_paths)

for image_file_path in list_of_file_paths:
    create_thumbnail(image_file_path)
