# https://www.steadylearner.com/blog/read/How-to-automatically-commit-files-to-GitHub-with-Python
import subprocess as cmd

cmd.run("rm -rf .git && git init && git add .", check=True, shell=True)
message = "Reset commits"
cmd.run(f"git commit -m '{message}'", check=True, shell=True)
cmd.run("git remote add origin git@github.com:steadylearner/Rust-Full-Stack.git", check=True, shell=True)
cmd.run("git push -u origin master -f", check=True, shell=True)


