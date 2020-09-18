FROM debian:buster-slim
ARG app_env=debug
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x tini \
 && mkdir -p /app/templates /app/public 
ADD /target/${app_env}/default-backend-rs /app/default-backend-rs
ADD /templates /app/default_templates
ADD /public /app/default_public
ADD /docker/entrypoint.sh /app/entrypoint.sh
ENTRYPOINT ["/tini", "--"]
RUN chown -R 1000:1000 /app
USER 1000
CMD ["/app/entrypoint.sh"]
EXPOSE 8000
VOLUME ["/app/templates", "/app/public"]
