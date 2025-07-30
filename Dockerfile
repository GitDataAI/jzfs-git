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

FROM nginx:alpine
COPY --from=backend-builder /app/target/release/jzfs /usr/local/bin/
RUN chmod +x /usr/local/bin/jzfs
COPY --from=frontend-builder /app/views/dist /usr/share/nginx/html
COPY script/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["sh", "-c", "/usr/local/bin/jzfs & nginx -g 'daemon off;'"]