from .convert_extension import csv_to_json, json_to_csv

# # You shouldn't have empty lines at the end in blog posts also.
def remove_meta(content: str) -> str:
    separator = "\n"
    remove_meta = content.split(separator)[8:] # Remove meta parts used for www.steadylearner.com
    content_without_meta = separator.join(remove_meta) # or content
    return content_without_meta




