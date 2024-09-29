FROM node:alpine AS ui-builder
WORKDIR /usr/src/observatory/ui

COPY ui/package.json ui/package-lock.json ./

RUN npm install
COPY ./ui/ ./
RUN npm run build

FROM rust:1.81 AS chef 
RUN cargo install cargo-chef 
WORKDIR /usr/src/observatory/service

FROM chef AS planner
COPY ./service .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/observatory/service/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./service .
COPY --from=ui-builder /usr/src/observatory/ui/dist ./ui
RUN cargo build --release --bin observatory 

FROM debian:bookworm-slim AS runtime

WORKDIR /usr/src/observatory/service
COPY --from=builder /usr/src/observatory/service/target/release/observatory /usr/src/app
ENTRYPOINT ["/usr/src/app/observatory"]