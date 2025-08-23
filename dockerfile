FROM rust:1.85 AS builder

WORKDIR /usr/src/app
COPY . .
RUN ls
RUN SQLX_OFFLINE=true cargo build --release

FROM debian:bookworm-slim
# 本地切换源
# RUN sed -i 's|deb.debian.org|mirrors.tuna.tsinghua.edu.cn|g' /etc/apt/sources.list.d/debian.sources
RUN apt-get update && apt-get install -y libpq5 
# && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/rstmplate /app/rstmplate
# COPY --from=builder /usr/src/app/config.toml /etc/oidc/
# RUN sed -E -i 's/password\s*=\s*"[^"]*"/password = ""/g' /app/oidc/config.toml
# COPY --from=builder /usr/src/app/templates /app/templates
WORKDIR /app

EXPOSE 4000
CMD ["/app/rstmplate"] 