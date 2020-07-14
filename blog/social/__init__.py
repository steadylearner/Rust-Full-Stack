import webbrowser as web
import tweepy
import click as cli

import sys
sys.path.append("..")
from settings import consumer_key, consumer_secret, access_token, access_token_secret


def to_github(payload: str):
    response = input("Do you really want to verify the result at GitHub?([y]/n)\n")

    if not response.startswith("n"):
        prefix = "https://github.com/steadylearner/blog/tree/master"
        web.open(f"{prefix}/{payload}")

def tweet(payload: str):
    tweet_response = input("Ready to share this post at Twitter?([y]/n)\n")

    if not tweet_response.startswith("n"):
        auth = tweepy.OAuthHandler(consumer_key, consumer_secret)
        auth.set_access_token(access_token, access_token_secret)
        api = tweepy.API(auth)

        title = payload.split("/")[-1].split(".")[0]
        title_for_link = title.replace(" ", "-")

        link = f"https://www.steadylearner.com/blog/read/{title_for_link}"

        image_response = input("Do you want to include an image?([y]/n)\n")

        if not image_response.startswith('n'):
            image_title = input("Path for it?(You can skip static/images)")
            entire_image_path = f"static/images/{image_title}"

            message = input("Message?\n")
            entire_tweet = f"I just published {title} at {link}\n {message}"

            #

            api.update_with_media(entire_image_path, entire_tweet)

            print(f"Send the tweet with\n message: {entire_tweet} \n image: {image_title}")
        else:
            message = input("Message?\n")
            entire_tweet = f"I just published {title} at {link}\n {message}"

            #

            api.update_status(entire_tweet)
            print(f"Send the tweet with\n message: {entire_tweet}")

# http://docs.tweepy.org/en/latest/api.html#API.user_timeline
def delete_latest_tweet():
    auth = tweepy.OAuthHandler(consumer_key, consumer_secret)
    auth.set_access_token(access_token, access_token_secret)
    
    api = tweepy.API(auth)
    latest_tweet_id = api.user_timeline("steadylearner_p")[0].id # https://twitter.com/steadylearner_p
    
    response = input("Do you really want to delete your lastest tweet?([y]/n)\n")

    if not response.startswith('n'):
        api.destroy_status(latest_tweet_id)
        print("Tweepy removed it.")