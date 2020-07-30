# I will start with Tkinter before I find something to code seriously.
# Refer to them.

# 1. https://realpython.com/python-gui-tkinter/
# 2. https://github.com/Dvlv/Tkinter-By-Example/blob/master/Tkinter-By-Example.pdf
# 3. https://docs.python.org/3/library/tk.html

# https://github.com/psf/black

# $pip install ipython black
# $black -q main.py

import tkinter as tk
window = tk.Tk()

label = tk.Label(text = "Target")
entry = tk.Entry()

label.pack()
entry.pack() # www.steadylearner.com

window.mainloop()
