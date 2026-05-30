FROM mcr.microsoft.com/devcontainers/rust:latest

RUN git clone --filter=blob:none --depth 1 --branch stable \
    https://github.com/flutter/flutter.git /opt/flutter

ENV PATH="/opt/flutter/bin:${PATH}"

RUN flutter config --enable-web --disable-analytics


RUN curl -1sLf 'https://dl.cloudsmith.io/public/task/task/setup.deb.sh' | sudo -E bash
RUN apt install task