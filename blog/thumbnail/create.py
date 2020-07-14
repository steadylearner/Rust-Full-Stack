import os
from PIL import Image

def create_thumbnail(infile: str, include_in_path: str = "thumbnail", size: tuple = (256, 256)):
    infile_name, infile_extension = os.path.splitext(infile)
    outfile = f"{infile_name}.{include_in_path}{infile_extension}" # .png, jpeg etc
    if infile.find(include_in_path) == -1:
        if infile != outfile:
            try:
                im = Image.open(infile)
                im.thumbnail(size)
                im.save(outfile, infile_extension.split(".")[1].upper()) # PNG, JPEG etc
                print(f"Pillow made the thumbnail of {infile}.")
            except IOError:
                print(f"cannot create thumbnail for {infile}")

# How to use it in frontend
# 1. When you use framework

# const useThumbnail = (path = "") => {
#   widthForThumbnail = "use the width you think adequate for this"
#   if (window.innerWidth > widthForThumbnail) {
#      // cosnsole.log(path);
#      return path;
#   } else {
#      const [file, extension] = path.split('.');
#      const includeInPath = "thumbnail";
#      const pathWithThumbnail = [file, includeInPath, extension].join('.');
#      // console.log(pathWithThumbnail);
#      return pathWithThumbnail;
#   }
# }

# src = "local/image.png" from the state 
# <img src={useThumbnail(state.src)} /> 

# 2. JavaScript only
# <img id="responsive-image" src="local/image.png" />

# widthForThumbnail = "use the width you think adequate for this"
# responsiveImage = document.getElementById("responsive-image")
# path = responsiveImage.src

# const useThumbnail = (path = "") => {
#     if window.innerWidth > widthForThumbnail {
#         responsiveImage.src = path
#     } else {
#         const [file, extension] = path.split('.');
#         const includeInPath = "thumbnail";
#         const pathWithThumbnail = [file, includeInPath, extension].join('.');
#         responsiveImage.src = pathWithThumbnail;
#     }
# }

# useThumbnail()