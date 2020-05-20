# Creating the __init__.py file means that the folder that includes it can be imported as a module from the parent directory.

def sum(arg):
    total = 0
    for val in arg:
        total += val
    return total
