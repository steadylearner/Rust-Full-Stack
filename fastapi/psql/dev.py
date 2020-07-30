# source ./bin/activate

import subprocess as cmd

response = input("[d]ev, [t]est?\n")

if response.startswith("t"):
    cp = cmd.run(f"pytest", check=True, shell=True)
else:
    cp = cmd.run(f"uvicorn main:app --reload", check=True, shell=True)

