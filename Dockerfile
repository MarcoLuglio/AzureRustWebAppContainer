# https://hub.docker.com/_/rust
FROM rust:alpine as builder

LABEL maintainer = "Marco Luglio <marcodejulho@gmail.com>"
COPY . /usr/src/main/

#RUN ls /usr/src/main

#default image workdir is /usr/src/myapp
WORKDIR /usr/src/main/
#RUN cargo install --path . \
#	&& cargo build
RUN cargo build --release --verbose

###############################################

FROM alpine

COPY --from=builder /usr/src/main/target/release/container_azure_rust /bin/main
# probably not the correct place to put these
COPY --from=builder /usr/src/main/hello.json /bin/hello.json
COPY --from=builder /usr/src/main/404.html /bin/404.html

RUN chmod u+x /bin/main \
	&& chmod u+r /bin/hello.json \
	&& chmod u+r /bin/404.html

WORKDIR /bin
EXPOSE 80
ENTRYPOINT [ "/bin/main" ]
CMD [""]