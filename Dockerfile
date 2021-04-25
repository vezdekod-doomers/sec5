FROM rust:buster
RUN apt-get update && apt-get install -y nano && rm -rf /var/lib/apt/lists/*
ENV EDITOR=/bin/nano
ADD ./ $HOME/src
RUN cd src/encoder && cargo build --release && cd ../decoder && cargo build --release