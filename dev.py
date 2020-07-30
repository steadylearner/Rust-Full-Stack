# https://www.steadylearner.com/blog/read/How-to-automatically-commit-files-to-GitHub-with-Python

import subprocess as cmd

# trace, debug, info, warn, error
# (https://github.com/seanmonstar/pretty-env-logger)

# You should also refer to these.
# (Use them while you are developing details.)
# https://doc.rust-lang.org/std/macro.eprintln.html
# https://doc.rust-lang.org/std/macro.dbg.html

response = input("Cargo [c]heck, [r]un, [b]inary file made before, [p]roduction or [l]og?\n")

if response.startswith("c"):
    cp = cmd.run(f"cargo watch -x 'check --bin main'", check=True, shell=True)
elif response.startswith("r"):
    cp = cmd.run(f"cargo watch -x 'run --bin main'", check=True, shell=True)
elif response.startswith("b"):
    cp = cmd.run(f"cp target/debug/main main && ./main", check=True, shell=True)
elif response.startswith("p"):
    cp = cmd.run(f"cargo run --bin main --release", check=True, shell=True)
else:
    log_type = input("What do you want to log(debug)?\n")
    default = "debug"

    if response.startswith("i"):
        log_type = "info"
    elif response.startswith("e"):
        log_type = "error"
    else:
        log_type = default

    cp = cmd.run(f"RUST_LOG={log_type} cargo run --bin main", check=True, shell=True)

