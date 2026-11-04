FROM ubuntu:24.04

# Meta
LABEL \
  "name"="ufc-ripper" \
  "maintainer"="Mahesh Bandara Wijerathna <m4heshd@gmail.com> (m4heshd)"

# Init
WORKDIR /ufcr

# Environment variables
ENV RUN_ENV=container

# Third-party media tools: use the distro ffmpeg/ffprobe. The bundled static
# ffmpeg build segfaults on some HLS streams; yt-dlp stays bundled.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Setup app
COPY ./package/linux/ .
RUN chmod +x ./ufc-ripper \
    && ln -sf /usr/bin/ffmpeg /ufcr/bin/ffmpeg \
    && ln -sf /usr/bin/ffprobe /ufcr/bin/ffprobe

# Ports
EXPOSE 8383

# Volumes
VOLUME ["/ufcr/config"]
VOLUME ["/downloads"]

# Start
CMD ["./ufc-ripper"]
