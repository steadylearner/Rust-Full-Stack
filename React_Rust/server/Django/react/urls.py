from django.urls import path
from react import views

urlpatterns = [
    path('', views.react, name='react'),
    path('user', views.react, name='react'),
]

