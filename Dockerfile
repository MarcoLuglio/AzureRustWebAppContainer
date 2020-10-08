# https://hub.docker.com/_/rust
FROM rust:alpine as builder

LABEL maintainer = "Marco Luglio <marcodejulho@gmail.com>"
#ENV GOOS "linux"
COPY . .

# verificar qual path
#RUN ls /
WORKDIR /usr/src/myapp
RUN cargo install --path .

###############################################

FROM alpine
COPY --from=builder /go/bin/main /bin/main
EXPOSE 80
ENTRYPOINT [ "/bin/main" ]
CMD [""]