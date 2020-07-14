# https://github.com/pallets/click
import click as cli
from settings import GITHUB_TOKEN
from github import Github

from post_crud import create, read, update, delete

# 3.
# def read(repo, path):
#     byte = repo.get_contents(path).decoded_content
#     cli.echo(byte.decode("utf-8"))

# http://click.palletsprojects.com/en/7.x/
@cli.command()
@cli.option(
    "--crud",
    "-c",
    prompt="Which CRUD process you want for the GitHub files?",
    default="c",
    help="Use c | r | u | d to manage files in your repository.",  # $python cli.py --help
)
@cli.option(
    "--lang",
    "-l",
    prompt="Programming language?(Python, Rust, JavaScript or py, rs, js)",
    default="Python",
    help="Use Python, Rust, JavaScript or py, rs, js etc",
)
@cli.option(
    "--filepath",
    "-f",
    prompt="The file path for it?",
    default="README.md",
    help="Type a path for the file you want to submit to GitHub.",
)
def manage_posts_at_github(crud: str, lang: str, filepath: str):
    target = "steadylearner/Python-Blog"
      # It should have equivalent folder structure in your machine

    g = Github(GITHUB_TOKEN)
    repo = g.get_repo(target)

    # 2. before extract it to function
    # byte = repo.get_contents(filepath).decoded_content
    # cli.echo(byte.decode("utf-8"))

    # 8. lang handler

    if lang == "py":
        lang = "Python"
    elif lang == "rs":
        lang = "Rust"
    elif lang == "js":
        lang = "JavaScript"
    else:
        pass

    # 5. conditional for CRUD after test it with create(repo, filepath)

    if crud == "c":
        create(repo, lang, filepath)
    elif crud == "r":
        read(repo, lang, filepath)
    elif crud == "u":
        update(repo, lang, filepath)
    elif crud == "d":
        delete(repo, lang, filepath)
    else:
        cli.echo("You should use c | r | u | d to manage GitHub files.")

    # 1. cli.echo(f"use {fp} for {filepath}")

if __name__ == "__main__":
    manage_posts_at_github()

# hint: Updates were rejected because the remote contains work that you do
# hint: not have locally. This is usually caused by another repository pushing
# hint: to the same ref. You may want to first integrate the remote changes

# $git push -u origin master -f
