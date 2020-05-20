from django.shortcuts import render

def react (request):
    return render(request, 'index.html', {})
