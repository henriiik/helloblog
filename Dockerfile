FROM rustlang/rust:nightly

ENV ROCKET_ENV=prod

WORKDIR /usr/src/helloblog

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo install

COPY ./pages ./pages
COPY ./static ./static

CMD ["helloblog"]
