FROM rust as builder
WORKDIR /usr/src/myapp
COPY . .
RUN rustc main.rs
CMD ["./main"]
