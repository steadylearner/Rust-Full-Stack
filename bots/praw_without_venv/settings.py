import os
from dotenv import load_dotenv
from pathlib import Path

import praw

# Search "How to use Python pathlib"(/ here equlas to os.path.join)
env_path = Path(".") / ".env"
load_dotenv(dotenv_path=env_path)

client_id = os.getenv("client_id")
client_secret = os.getenv("client_secret")
password = os.getenv("password")

username = "steadylearner_p"

def login():
    reddit = praw.Reddit(
        client_id=client_id,
        client_secret=client_secret,
        password=password,
        user_agent=f"/u/{username}",
        username=username,
    )
    return reddit
