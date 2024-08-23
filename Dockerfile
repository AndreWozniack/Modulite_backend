# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.80.1
ARG APP_NAME=modulite


FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

# Install host build dependencies.
RUN apk add --no-cache openssl-dev
RUN apk add --no-cache openssl-libs-static
RUN apk add --no-cache clang lld musl-dev git

# Build the application.
RUN --mount=type=bind,source=src,target=src \
 --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
 --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
 --mount=type=cache,target=/app/target/ \
 --mount=type=cache,target=/usr/local/cargo/git/db \
 --mount=type=cache,target=/usr/local/cargo/registry/ \
 cargo build --locked --release && \
 cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser \
 --disabled-password \
 --gecos "" \
 --home "/nonexistent" \
 --shell "/sbin/nologin" \
 --no-create-home \
 --uid "${UID}" \
 appuser
USER appuser

# Se você quiser manter a variável DATABASE_URL na imagem final
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

COPY --from=build /bin/server /bin/

EXPOSE 8080

CMD ["/bin/server"]

