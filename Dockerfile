# https://hub.docker.com/_/rust
FROM rust:alpine as builder

LABEL maintainer = "Marco Luglio <marcodejulho@gmail.com>"
COPY . /usr/src/main/

RUN ls /usr/src

#default image workdir is /usr/src/myapp
WORKDIR /usr/src/main/
#RUN cargo install --path . \
#	&& cargo build
RUN cargo build --release --verbose \
	&& chmod u+x /usr/src/main/target/release/container_azure_rust

###############################################

FROM alpine
COPY --from=builder /usr/src/main/target/release/container_azure_rust /bin/main
# probably not the correct place to put these
COPY --from=builder /usr/src/main/hello.json /bin/hello.json
COPY --from=builder /usr/src/main/404.html /bin/404.html
EXPOSE 80
RUN cd /bin
ENTRYPOINT [ "/bin/main" ]
CMD [""]