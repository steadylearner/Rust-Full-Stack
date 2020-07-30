# https://tinydb.readthedocs.io/en/latest/getting-started.html
# Make a Rust version also and compare them.
# https://docs.rs/tinydb/0.0.7/tinydb/struct.Database.html#method.query_item

from tinydb import TinyDB, Query

contacts = TinyDB("contacts.json")
Contact = Query()

response = input("Tinydb [c]reate, [r]ead, [u]pdate, [d]elete, [e]mpty or [l]ist?\n")

# Suppose that there is no duplicate data for email.
if response.startswith("c"):
    email = input("What is the email?\n")
    author = input("Who is the author?\n")

    contacts.insert({"email": email, "author": author})
    print(f"{email} is saved at contacts.json")
elif response.startswith("r"):
    author = input("Who is the author of the email you want to find?\n")

    print(contacts.search(Contact.author == author))
elif response.startswith("u"):
    author = input("Who is the author of the email you want to update?\n")
    email = input("What is the new email?\n")

    contacts.update({"email": email}, Contact.author == author)
    print(f"The new email {email} is saved for the {author} at contacts.json")
elif response.startswith("d"):
    email = input("Which email you want to delete?\n")

    contacts.remove(Contact.email == email)
elif response.startswith("e"):
    contacts.truncate()
else:
    index = 1
    for contact in contacts:
        email = contact["email"]
        author = contact["author"]
 
        print(f"{index}. {email} ({author})\n")
        
        index = index + 1 
