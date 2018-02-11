FROM rustlang/rust:nightly

ENV ROCKET_ENV=prod

WORKDIR /usr/src/helloblog
COPY . .

RUN cargo install

CMD ["helloblog"]
