FROM node:alpine as ui-builder
WORKDIR /usr/src/observable/ui
COPY ui/package.json ui/package-lock.json ./
RUN npm install
COPY ./ui/ ./
RUN npm run build
CMD ["npm", "run", "dev", "--", "--host"]


# FROM rust:1.81 as service-builder
# WORKDIR /usr/src/observable/service
# COPY ./service .
# RUN cargo build --release

# FROM debian:bullseye-slim

# COPY --from=service-builder /usr/local/cargo/bin/observable /usr/local/bin/observable
# CMD ["observable"]
