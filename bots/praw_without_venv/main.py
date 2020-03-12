# $pip freeze > requirements.txt to extract the list of them.
# $pip install -r requirements.txt to install them later.

import os
import praw  # https://www.reddit.com/prefs/apps/
from termcolor import colored  # https://pypi.org/project/termcolor/

from settings import login

reddit = login()

while True:

    target = input("Which subreddit you want to scrap?\n")

    # Use switch or if else for them.
    # forhire, RemoteJobs, remotejobr ,rust, goalng, python
    if not target:
        target = "forhire"

    subreddit = reddit.subreddit(target)

    limit = input("How many you want?\n")

    if not limit:
        limit = 50
    else:
        limit = int(limit)

    content = f"# Latest {limit} posts from {target} subreddit. \n\n"

    def reddit_prefix_check(path: str):
        if path.startswith("/r/"):
            return "https://www.reddit.com" + path
        else:
            return path

    index = 1  # The code below is not iterable object so use this
    for submission in subreddit.new(limit=limit):
        # print(dir(submission)) # Use this to write more features.

        # It(visted) doesn't update fast? Remove it if you think it unnecessary.
        if not submission.visited:
            # requirements = "[HIRING]" in submission.title.upper()
            # print(requirements)

            freelance_relevant_subreddits = ["forhire", "RemoteJobs"]

            if target in freelance_relevant_subreddits:
                if "[HIRING]" in submission.title.upper():
                    print(f"{index}. {submission.title}({colored(reddit_prefix_check(submission.url), 'blue')})")
                    
                    payload = f"{index}. [{submission.title}]({reddit_prefix_check(submission.url)})"
                    content += payload + "\n"
                    index += 1
            else:
                print(f"{index}. {submission.title}({colored(reddit_prefix_check(submission.url), 'blue')})")
                
                payload = f"{index}. [{submission.title}]({reddit_prefix_check(submission.url)})"
                content += payload + "\n"
                index += 1

    # https://github.com/steadylearner/Rust-Full-Stack/blob/master/blog/post_log/create.py
    # filename = f"new_{target}_posts.md"

    filename = f"{target}.md"
    save = input(f"\nDo you want save it to {filename}?([n]/y])\n")

    if save.startswith("y"):
        with open(filename, "w") as f:
            f.write(content)
            print(f"\nThe {filename} was built.")

            # end = timer()
            # time_passed = end - start
            # print(f"\nThe {filename} was built in {round(time_passed, 1)} seconds.")
    else:
        print(f"End scraping {target} subreddit.")

    end = input(f"\nContinue?([y]/n])\n")

    if end.startswith("n"):
        break

# freelancer = input(f"\nFreelancer?([y]/n])\n")
# if freelancer.startswith("n"):
# keyword = "[HIRING]"
# else:
# keyword = "[FOR HIRE]"
