FROM rust:1-alpine

# RUN apk add pkgconfig openssl-dev musl-dev libressl-dev
RUN apk add pkgconfig musl-dev libressl-dev

WORKDIR /usr/src/hopitude_to_ical
COPY . .

RUN cargo install --path .

CMD ["hopitude_to_ical"]

