# Use __init__.py for the file name (if you want to publish the package.)

from vibora import Vibora, Response
from vibora.static import StaticHandler

import re

app = Vibora(
    static=StaticHandler(
        paths=['../public'],
        url_prefix='',
    ),
)

# https://docs.vibora.io/routing, /\/+(user)?$/
@app.route('/')
async def user():
    single_page_app = "../public/index.html"
    with open(single_page_app, 'rb') as input_file: 
        data = input_file.read()
        return Response(data, headers={'content-type': 'html'})

@app.route('/user')
async def user():
    single_page_app = "../public/index.html"
    # https://docs.vibora.io/responses
    with open(single_page_app, 'rb') as input_file: 
        data = input_file.read()
        return Response(data, headers={'content-type': 'html'})

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=8000)


