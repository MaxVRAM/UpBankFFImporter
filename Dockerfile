FROM rust:1-alpine
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./LICENSE.md ./LICENSE.md
COPY ./config/settings.yaml ./config/settings.yaml
RUN apk add musl-dev musl
RUN cargo install -v --path .
CMD up_bank_fidi
