FROM debian
RUN apt update && apt install -y rustc
COPY *.rs /tmp/
RUN cd /tmp && rustc main.rs
ENTRYPOINT /tmp/main