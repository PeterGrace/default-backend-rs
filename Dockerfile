FROM debian:buster-slim
ARG app_env=debug
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x tini \
 && mkdir -p /app/templates /app/public 
ADD /target/${app_env}/default-backend-rs /app/default-backend-rs
ADD /docker/entrypoint.sh /app/entrypoint.sh
ENTRYPOINT ["/tini", "--"]
CMD ["/app/entrypoint.sh"]
EXPOSE 8000
