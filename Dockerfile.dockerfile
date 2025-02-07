FROM ubuntu:latest
WORKDIR /app
COPY ./app
RUN apt-get update && apt-get install -y \
    libx11-dev \
    libxrandr-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libgles2-mesa-dev \
    libwayland-dev \
    libgbm-dev \
    libdrm-dev \
    libxcb-dev \
    libxcb-xkb-dev \
    libxcb-icccm-dev \
    libxcb-image-dev \
    libxcb-keysyms-dev \
    libxcb-render-dev \
    libxcb-shape-dev \
    libxcb-sync-dev \
    libxcb-util-dev \
    libxcb-xfixes-dev \
    libxcb-xinerama-dev \
    libxcb-xinput-dev \
    libxcb-xkb-dev \
    && rm -rf /var/lib/apt/lists/*
RUN groupadd -r app && useradd -r -g app -G sudo -m -d /app -s /bin/bash app
USER app
EXPOSE 8085
CMD [""]