import tkinter as tk


border_effects = {
    "flat": tk.FLAT,
    "sunken": tk.SUNKEN,
    "raised": tk.RAISED,
    "groove": tk.GROOVE,
    "ridge": tk.RIDGE,
}


window = tk.Tk()


for relief_name, relief in border_effects.items():

    frame = tk.Frame(master=window, relief=relief, borderwidth=5)

    frame.pack(side=tk.LEFT)

    label = tk.Label(master=frame, text=relief_name)

    label.pack()


window.mainloop()
