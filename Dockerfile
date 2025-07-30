FROM rust:latest AS backend-builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libssl-dev && cargo build --release --bin jzfs

FROM node:18-alpine AS frontend-builder
WORKDIR /app/views
COPY views/package.json views/pnpm-lock.yaml ./
RUN apk add --no-cache git && npm install -g pnpm && pnpm install
COPY views/ .
RUN pnpm build

FROM ubuntu:22.04
COPY --from=backend-builder /app/target/release/jzfs /explore
RUN chmod +x /usr/local/bin/jzfs
COPY --from=frontend-builder /app/views/dist /explore/html
EXPOSE 80
CMD ["sh", "-c", "/usr/local/bin/jzfs & nginx -g 'daemon off;'"]