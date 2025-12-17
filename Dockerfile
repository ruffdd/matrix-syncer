From debian
RUN apt update && apt install -y rustc
copy *.rs /tmp/
RUN cd /tmp && rustc main.rs
ENTRYPOINT /tmp/main