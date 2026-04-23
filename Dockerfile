FROM rust:1-alpine3.21 AS build

ARG pkg=website

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

RUN set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

FROM alpine:3.21

WORKDIR /app
RUN chmod 777 /app

COPY --from=build /build/main ./

COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/socials.txt ./

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8002

RUN ls -la

EXPOSE 8002

CMD ./main

