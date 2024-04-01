ARG VERSION=alpine
FROM python:${VERSION}

COPY . /docs

VOLUME [ "/docs" ]

RUN apk update && \
    apk add bash gcc graphviz make musl-dev ttf-opensans

RUN pip install diagrams

WORKDIR /docs

ENTRYPOINT [ "/bin/bash" ]