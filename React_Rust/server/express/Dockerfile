FROM steadylearner/ubuntu_node
WORKDIR /app
COPY . /app
# EXPOSE 8000, CMD ["yarn", "serve"] for local
# EXPOSE 80, CMD ["yarn", "build"] for AWS 
EXPOSE 80
CMD ["yarn", "serve"]

