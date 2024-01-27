FROM golang:1.21.6-alpine3.18 as builder

RUN apk update && apk add --no-cache make && apk --update add ca-certificates && apk add --no-cache git

ENV USER=appuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIER /math_server

COPY go.mod go.sum /math_server/

RUN go mod download

COPY . /math_server/

RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 make build

FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /math_server/math_server /

USER appuser:appuser

ENTRYPOINT ["/math_server"]
