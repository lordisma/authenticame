FROM rust:1.65-buster as builder

WORKDIR /usr/service/
COPY . .

RUN cargo install --path .

FROM debian:buster

COPY --from=builder /usr/local/cargo/bin/authenticame /usr/local/bin/authenticame
CMD [ "authenticame" ]