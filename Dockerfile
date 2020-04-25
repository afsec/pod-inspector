FROM busybox:musl
COPY ./dist/pod-inspector /
ENV PIPELINE development
EXPOSE 8081
ENTRYPOINT ["/pod-inspector"]
