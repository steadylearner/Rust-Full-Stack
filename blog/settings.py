from dotenv import load_dotenv
from pathlib import Path
import os

# Search "How to use Python pathlib"(/ here equlas to os.path.join)
env_path = Path('.') / '.env'
load_dotenv(dotenv_path=env_path)

GITHUB_TOKEN = os.getenv("GITHUB_TOKEN")
# print(f"My GitHub Token is {GITHUB_TOKEN} and I will build a blog with it.")

# Twitter

consumer_key = os.getenv("consumer_key")
consumer_secret = os.getenv("consumer_secret")
access_token = os.getenv("access_token")
access_token_secret = os.getenv("access_token_secret")

# email

email_author = os.getenv("email_author")



