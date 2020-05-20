# https://www.google.com/search?&q=what+is+ipp+protocol
# https://github.com/dremon/ipp.rs

# https://stackoverflow.com/questions/9886274/how-can-i-convert-canvas-content-to-an-image
# https://stackoverflow.com/questions/42726925/python-how-to-communicate-with-printer

import os

printer_name = "your_printer_name"
file_name = "your_file_name"

# $man lpr
os.system(f"lpr -P {printer_name} {file_name}")

# https://stackoverflow.com/questions/36628164/how-to-send-print-job-to-printer-in-python

# https://smallbusiness.chron.com/sending-things-printer-python-58655.html
# Note that "printer_name" represents the name of the printer you use on your system and will vary. "file_name.txt" is the name of the text file used for printing and will also vary.
