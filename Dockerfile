# Stage 1: Build the Rust API
FROM rust:latest AS rust_builder

# Create a work directory for the Rust API
WORKDIR /app/api

# Copy the Rust API source code into the container
COPY conjugation-api/ .

# Build the Rust API
RUN cargo build --release

# Stage 2: Build the Bun UI
FROM node:latest AS ui_builder

RUN npm install -g bun

# Create a work directory for the Bun UI
WORKDIR /app/ui

# Copy the Bun UI source code into the container
COPY conjugation-ui/ .

# Install dependencies and build the Bun UI
RUN bun install
RUN bun run build

# Stage 3: Final Image
FROM debian:latest

# Install Supervisor
RUN apt-get update && apt-get install -y supervisor nodejs

# Create a work directory for the final application
WORKDIR /app

# Copy the built Rust API from the Rust builder stage
COPY --from=rust_builder /app/api/target/release/conjugation-api ./app/api/
COPY --from=rust_builder /app/api/data ./data

# Copy the built Bun UI from the UI builder stage
COPY --from=ui_builder /app/ui/build ./ui/build
COPY --from=ui_builder /app/ui/package.json ./ui/package.json
COPY --from=ui_builder /app/ui/node_modules ./ui/node_modules

RUN echo "[supervisord]" >> /etc/supervisor/conf.d/app.conf
RUN echo "logfile=/dev/stdout" >> /etc/supervisor/conf.d/app.conf
RUN echo "logfile_maxbytes=0" >> /etc/supervisor/conf.d/app.conf
RUN echo "loglevel=info" >> /etc/supervisor/conf.d/app.conf
RUN echo "pidfile=/tmp/supervisord.pid" >> /etc/supervisor/conf.d/app.conf
RUN echo "nodaemon=true" >> /etc/supervisor/conf.d/app.conf
RUN echo "user=root" >> /etc/supervisor/conf.d/app.conf

RUN echo "[unix_http_server]" >> /etc/supervisor/conf.d/app.conf
RUN echo "file=/tmp/supervisor.sock" >> /etc/supervisor/conf.d/app.conf

# Create a Supervisor configuration file to run both processes
RUN echo "[program:api]" >> /etc/supervisor/conf.d/app.conf
RUN echo "command=./app/api/conjugation-api" >> /etc/supervisor/conf.d/app.conf
RUN echo "autostart=true" >> /etc/supervisor/conf.d/app.conf
RUN echo "[program:ui]" >> /etc/supervisor/conf.d/app.conf
RUN echo "command=node ./ui/build/" >> /etc/supervisor/conf.d/app.conf
RUN echo "autostart=true" >> /etc/supervisor/conf.d/app.conf

# Expose ports if needed (adjust as per your requirements)
EXPOSE 8080
EXPOSE 3000

# Start Supervisor to manage both processes
CMD ["supervisord", "-c", "/etc/supervisor/supervisord.conf"]
