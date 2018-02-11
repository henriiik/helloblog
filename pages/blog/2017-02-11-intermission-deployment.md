## [Intermission: Deployment](/blog/2017-02-11-intermission-deployment)

2017-02-11 by Henrik Sjööh

## The Container

I fiddled around with trying to build self-contained binaries for a while but in the end I settled with using the official docker image for rust and creating a `Dockerfile` for the project.

```Dockerfile
FROM rustlang/rust:nightly

ENV ROCKET_ENV=prod

WORKDIR /usr/src/helloblog
COPY . .

RUN cargo install

CMD ["helloblog"]
```

It's now possible to run the project in docker!

```
$ docker build . -t helloblog
$ docker run -d -p 80:80 helloblog:latest
```

## The Server

I decided to deploy site to a digital ocean droplet. After some fiddling I got docker machine working with the digital ocean driver by following the instructions in [the docker-machine docs](https://docs.docker.com/machine/drivers/digital-ocean/).

```
docker-machine create --driver digitalocean do
eval $(docker-machine env do)
```

It works!

## It's Alive!

You can now visit the site on [https://helloblog.enhenrik.nu](https://helloblog.enhenrik.nu)

## It's not production ready

I would just like to note that this is still just a toy project and I don't recommend running any service that any person depends on like this. 😅
