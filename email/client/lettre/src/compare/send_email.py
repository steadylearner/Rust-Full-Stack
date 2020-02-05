# https://myaccount.google.com/lesssecureapps
# $pip3 install yagmail[all]

# Refer to these if you want to write your own Python implementation instead
# 1. http://www.pybloggers.com/2018/12/sending-emails-with-python/
# 2. https://realpython.com/python-send-email/

# Do not use email.py for filename for it will block you to use email relevant built in packages(No file name equal to built in modules or packages you use)

# Oauth with
# https://developers.google.com/gmail/api/quickstart/python
# if you want more security

# Use paid plan with https://gsuite.google.com/pricing.html
# or https://github.com/tomav/docker-mailserver

import sys
sys.path.append("..")

import yagmail
from termcolor import colored

from settings import GITHUB_TOKEN, email_author
from draft import body # use files in templates instead of this while you refer to the Rust version.

yag = yagmail.SMTP(email_author) # email_author will be youremail in youremail@email.com

subject = input("What is the subject?\n")

colored_comma = colored(',', attrs=["bold"])
to = input(f"Who will receive this email?(Use {colored_comma} to send the email for many users)\n")

colored_options = colored("(pt|en)", attrs=["bold"])
response = input(f"What language you want to use for the mail?{colored_options}\n")

resume_en = "files/cv-en.pdf"
resume_pt = "files/cv-pt.pdf"
img = "static/favicon.ico"

contents = [
    body[response],
    resume_pt,
    resume_en,
    img,
]

yag.send(
    to = to.split(", "),
    subject = subject,
    contents = contents
)
