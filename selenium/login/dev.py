# https://www.steadylearner.com/blog/read/How-to-automatically-commit-files-to-GitHub-with-Python

import subprocess as cmd

# trace, debug, info, warn, error
# (https://github.com/seanmonstar/pretty-env-logger)

# You should also refer to these.
# (Use them while you are developing details.)
# https://doc.rust-lang.org/std/macro.eprintln.html
# https://doc.rust-lang.org/std/macro.dbg.html

response = input("Cargo [c]heck, [r]un, reuse [b]inary file made before or build [p]roduction file?\n")

if response.startswith("c"):
    cp = cmd.run(f"cargo watch -x 'check'", check=True, shell=True)
elif response.startswith("r"):
    cp = cmd.run(f"cargo watch -x 'run'", check=True, shell=True)
elif response.startswith("b"):
    cp = cmd.run(f"cp target/debug/login . && ./login", check=True, shell=True)
else:
    cp = cmd.run(f"cargo run --release", check=True, shell=True)

