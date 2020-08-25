# 1. With Linux

# https://www.cyberciti.biz/faq/how-do-i-compress-a-whole-linux-or-unix-directory

# $tar -zcvf test.tar.gz test to compress the directory
# $tar -zxvf test.tar.gz to decompress the directory

# Then, $cd test && find . | gzip -d *.gz to unzip all files in the current directory
# $find . | gzip * to zip all files or *.ext for files with the specific extension(ext). For example, .jpg, .png and others.

# 2. With Python

# 1) File -> https://github.com/steadylearner/code/blob/master/reference/Python/gzip/gzip_decompresser.py
# 2) Directory -> https://github.com/steadylearner/code/blob/master/reference/Python/gzip/gzip_directory_decompresser.py

# or https://docs.python.org/3/library/gzip.html or https://www.google.com/search?q=how+to+use+gzip+python
