# From rust
FROM ubuntu
WORKDIR /app
COPY ./public /app/public
COPY ./target/release/react_actix /app
EXPOSE 8000 
CMD ["./react_actix"]

