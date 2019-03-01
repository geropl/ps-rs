FROM gitpod/workspace-full:latest

USER root
RUN apt-get update && apt-get install -yq \
        uidmap \
        iptables \
&& apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/*

USER gitpod
RUN  bash -lc "cargo install cargo-watch"

USER root
