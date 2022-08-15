
FROM rust:latest as builder
WORKDIR /usr/src/ghems
COPY . .
RUN cargo install --path .

FROM scratch

