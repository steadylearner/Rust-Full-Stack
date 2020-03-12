# 6.
import sys
import os
import time

import click as cli # ipython and use this command then, inspect its methods(Use cli.clear() if you want)
from termcolor import colored

from social import to_github, tweet
from post_log import create_post_log

# 4.

"""
# Why use filepath instead of file, file_location, path etc
#in def github(crud, filepath) and other functions in crud.py?

# [C] PyGithub use "path" and "file" in its API such as contents.path delete.file etc.
# Therefore, use filepath instead one of them here.

# path

# The name of a file or directory, a location in a file system.
# It is expressed in a string of characters separated by a delimiting character.
"""

def create(repo: object, lang: str ,filepath: str):
    colored_filepath = colored(filepath, attrs=['bold'])
    response = input(f"Do you really want to commit {colored_filepath} to GitHub?([y]/n)") # or (Y/n)

    if response.startswith("n"):
        print(f"\nIt won't commit {colored_filepath}.")
    else:
        # https://book.pythontips.com/en/latest/open_function.html
        payload = f"posts/{lang}/{filepath}"
        with open(payload, "r") as f:
            contents = f.read()
            if len(contents) == 0:
                print("The filepaths shouldn't be empty. Verify it has some codes in it first.")
                # f.close() with handles error and close() etc instead of you, similar to ? in Rust
            else:
                repo.create_file(payload, f"create {payload}", contents, branch = "master")
                print(f"\n\t You submitted {payload} to GitHub\n")
                print(f"\n\t The contents is \n {contents}")

                # to_github(payload)

                title = filepath.split(".")[0]
                create_post_log(lang, title, "C")

                # share_response = input(f"Do you want to share {colored_filepath} with others also?([y]/n)")
                # if not share_response.startswith("n"):
                    # tweet(payload)
                    # email_to_subscribers()

# [R]
def read(repo: object, lang: str, filepath: str):
    payload = f"posts/{lang}/{filepath}"
    byte = repo.get_contents(payload).decoded_content
    print(f"\n{byte.decode('utf-8')}")

# [U]
def update(repo: object, lang: str, filepath: str):
    colored_filepath = colored(filepath, attrs=['bold'])
    response = input(f"Do you really want to update {colored_filepath} at GitHub?([y]/n)")

    if response.startswith("n"):
        print(f"\nIt won't update {colored_filepath}.")
    else:
        payload = f"posts/{lang}/{filepath}"
        with open(payload, "r") as f:
            new = f.read()
            old = repo.get_contents(payload).decoded_content

            # print(type(old)) => byte
            # print(type(new)) => str

            if old == str.encode(new):
                print("The contents is not updated. Edit it first.")
            else:
                contents = repo.get_contents(payload, ref="master")
                repo.update_file(contents.path, f"update {payload}", new, contents.sha, branch="master")

                print(f"GitHub updated {payload}\n")
                print(f"The updated file is \n {new}")

                title = filepath.split(".")[0]
                create_post_log(lang, title, "U")

                # Use web.open if you want

# [D]
def delete(repo: object, lang: str, filepath: str):
    colored_filepath = colored(filepath, attrs=['bold'])
    response = input(f"Do you really want to delete {colored_filepath}?([y]/n)")

    payload = f"posts/{lang}/{filepath}"

    if response.startswith("n"):
        print(f"\nIt won't delete {colored(payload, attrs=['bold'])}.")
    else:
        os.remove(payload)
        print(f"{colored(payload, attrs=['bold'])} removed")

        contents = repo.get_contents(payload, ref="master")
        repo.delete_file(contents.path, f"remove {payload}", contents.sha, branch="master")
        print("It is also removed from the GitHub")

        title = filepath.split(".")[0]
        create_post_log(lang, title, "D")
