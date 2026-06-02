FROM mcr.microsoft.com/devcontainers/rust:latest

# Install Task runner
RUN curl -1sLf 'https://dl.cloudsmith.io/public/task/task/setup.deb.sh' | bash \
  && apt-get install -y task

# Install Node.js via nvm (LTS)
ENV NVM_DIR=/usr/local/nvm
ENV NODE_VERSION=lts/*
RUN mkdir -p "$NVM_DIR" \
  && curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash \
  && . "$NVM_DIR/nvm.sh" \
  && nvm install "$NODE_VERSION" \
  && nvm alias default "$NODE_VERSION" \
  && nvm use default \
  && ln -sf "$NVM_DIR/versions/node/$(ls $NVM_DIR/versions/node/)/bin/node" /usr/local/bin/node \
  && ln -sf "$NVM_DIR/versions/node/$(ls $NVM_DIR/versions/node/)/bin/npm" /usr/local/bin/npm \
  && ln -sf "$NVM_DIR/versions/node/$(ls $NVM_DIR/versions/node/)/bin/npx" /usr/local/bin/npx

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash \
  && ln -sf /root/.bun/bin/bun /usr/local/bin/bun \
  && ln -sf /root/.bun/bin/bunx /usr/local/bin/bunx

# Verify
RUN node -v && npm -v && bun --version && task --version