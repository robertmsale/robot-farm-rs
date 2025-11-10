# syntax=docker/dockerfile:1.10

FROM buildpack-deps:trixie

ENV DEBIAN_FRONTEND=noninteractive \
    TZ=UTC \
    PNPM_HOME=/root/.local/share/pnpm \
    PATH=/root/.local/share/pnpm:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        jq \
        libsqlite3-dev \
        python3 \
        python3-pip \
        ripgrep \
        sqlite3 \
        tini

# Install Node.js 20.x from NodeSource rather than Debian's older channel.
RUN mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    printf "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main\n" > /etc/apt/sources.list.d/nodesource.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends nodejs

RUN corepack enable && \
    corepack prepare pnpm@9.12.2 --activate && \
    npm install -g @openai/codex@0.56.0

WORKDIR /workspace

RUN mkdir -p /opt/robot-farm
COPY .robot-farm/cache/final_message.schema.json /opt/robot-farm/final_message.schema.json

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["bash"]
