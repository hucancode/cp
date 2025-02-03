FROM amd64/alpine:latest
RUN apk update && apk add --no-cache \
    nasm \
    gcc \
    libc-dev \
    make \
    && rm -rf /var/cache/apk/*
WORKDIR /workspace
CMD ["/bin/sh"]
