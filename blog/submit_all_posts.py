# https://stackoverflow.com/questions/38594717/how-do-i-push-new-files-to-github
# Use it before you test this project

import glob

from settings import GITHUB_TOKEN

from github import Github, InputGitTreeElement

from termcolor import colored

target = "steadylearner/Python-Blog" # It should have equivalent folder structure in your machine
g = Github(GITHUB_TOKEN)
repo = g.get_repo(target)

# Use listdir or walk later
# refer/traverse_dir.py/list_of_file_paths
python_post_list = glob.glob(("posts/Python/*.md"))
rust_post_list = glob.glob("posts/Rust/*.md")

commit_message = "Submit all blog posts for Python and Rust"

posts = python_post_list + rust_post_list # or [*python_post_list, *rust_post_list]
print(posts)

master_ref = repo.get_git_ref('heads/master')
master_sha = master_ref.object.sha
base_tree = repo.get_git_tree(master_sha)

element_list = list()
for entry in posts:
    with open(entry, 'r') as input_file: # rb when you want to use image or other binary files
        data = input_file.read()
        element = InputGitTreeElement(entry, '100644', 'blob', data)
        element_list.append(element)

tree = repo.create_git_tree(element_list, base_tree)
parent = repo.get_git_commit(master_sha)
commit = repo.create_git_commit(commit_message, tree, [parent])
master_ref.edit(commit.sha)