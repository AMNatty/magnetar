FROM docker.io/rust:1.69-bullseye as build

RUN update-ca-certificates

ENV USER=magnetar
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /magnetar

COPY ./ .

RUN cargo build --release

FROM docker.io/debian:bullseye-slim

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

WORKDIR /magnetar

RUN chown -R 10001:10001 .

COPY --from=build /magnetar/target/release/magnetar ./

USER magnetar:magnetar

EXPOSE 4939/tcp

CMD ["/magnetar/magnetar"]