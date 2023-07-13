FROM rust

COPY . /home/rust-api
WORKDIR /home/rust-api
RUN ["cargo", "build"]

EXPOSE 8080
CMD ["cargo", "run"]
