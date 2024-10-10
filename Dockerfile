FROM rust:latest AS build

COPY . .

RUN cargo build --release --verbose

FROM debian:stable-slim

COPY --from=build /target/release/usthing-backend-test /bin/

CMD ["usthing-backend-test"]

EXPOSE 80