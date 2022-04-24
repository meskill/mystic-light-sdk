FROM dev:rust

USER root

RUN rustup component add rustfmt && \
    rustup component add clippy && \
    rustup target add x86_64-pc-windows-gnu && \
    apk add --no-cache mingw-w64-gcc musl-dev && \
    ln -s /usr/x86_64-w64-mingw32/lib/liboleaut32.a /usr/x86_64-w64-mingw32/lib/libOleAut32.a

USER rust