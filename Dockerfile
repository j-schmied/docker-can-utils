FROM alpine:latest

WORKDIR /data

RUN apk update
RUN apk upgrade
RUN apk add --no-cache make cmake linux-headers can-utils coreutils python3

#RUN modprobe can
#RUN modprobe vcan
#RUN modprobe peak-usb
#RUN ip link set can0 up type can bitrate 500000
#RUN link set up can0

