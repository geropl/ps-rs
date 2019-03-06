FROM gitpod/workspace-full:latest

USER root
RUN apt-get update && apt-get install -yq \
        # Docker rootless
        uidmap \
        iptables \
&& apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/*

# From: https://github.com/fornwall/rust-static-builder/blob/master/Dockerfile
# -DOPENSSL_NO_SECURE_MEMORY needed due to https://github.com/openssl/openssl/issues/7207
RUN cd /tmp && OPENSSL_VERSION=1.1.1a && \
    curl -LO "https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz" && \
    tar xf "openssl-$OPENSSL_VERSION.tar.gz" && cd "openssl-$OPENSSL_VERSION" && \
    env CC=musl-gcc ./Configure \
        no-shared no-zlib no-engine no-unit-test -DOPENSSL_NO_SECURE_MEMORY \
        -fPIC --prefix=/usr/local/musl linux-x86_64 && \
    env C_INCLUDE_PATH=/usr/local/musl/include/ make depend && \
    make install_sw

RUN echo "export OPENSSL_DIR=/usr/local/musl/" >> /home/gitpod/.bashrc && \
    echo "export OPENSSL_INCLUDE_DIR=/usr/local/musl/include/" >> /home/gitpod/.bashrc && \
    echo "export OPENSSL_LIB_DIR=/usr/local/musl/lib/" >> /home/gitpod/.bashrc && \
    echo "export OPENSSL_STATIC=1" >> /home/gitpod/.bashrc

USER root
