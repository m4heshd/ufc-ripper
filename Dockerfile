FROM ubuntu:22.04

# Meta
LABEL \
  "name"="ufc-ripper" \
  "maintainer"="Mahesh Bandara Wijerathna <m4heshd@gmail.com> (m4heshd)"

# Init
WORKDIR /ufcr

# Environment variables
ENV RUN_ENV=container

# Setup app
COPY ./package/linux/ .
RUN chmod +x ./ufc-ripper

# Ports
EXPOSE 8383

# Volumes
VOLUME ["/ufcr/config"]
VOLUME ["/downloads"]

# Start
CMD ["./ufc-ripper"]
