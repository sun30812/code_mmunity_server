FROM rust
ENV DB_SERVER 'localhost'
ENV DB_PORT 3306
ENV DB_USER user
ENV DB_PASSWD 0000
ENV DB_DATABASE test
ENV USE_SSL false
ENV APP_PORT 8080

ADD . /code_mmunity_server
WORKDIR /code_mmunity_server
RUN cargo build --release
RUN cp target/release/code_mmunity_server /code_mmunity_server.out
# RUN cp -r cert /cert
WORKDIR /
RUN rm -rf /code_mmunity_server
CMD ./code_mmunity_server.out
