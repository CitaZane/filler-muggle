FROM rust:1.63-buster

WORKDIR /tmp/filler
RUN curl -s https://assets.01-edu.org/filler/filler.zip -o filler.zip
RUN unzip filler.zip && rm filler.zip
RUN mv docker_image /filler

COPY ./robot /tmp/filler/robot

WORKDIR /tmp/filler/robot
RUN cargo build --release --bin robot

RUN mv target/release/robot /filler/robots/muggle
RUN rm -r /tmp/filler
RUN chmod +x /filler/game_engine
RUN chmod +x /filler/robots/*

WORKDIR /filler/
ENTRYPOINT /bin/bash