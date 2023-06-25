# Services

Twilight is built with a service-minded approach. This means that it caters to
both monolithic and multi-serviced applications equally. If you have a very
large bot and have a multi-serviced application and feel like Rust is a good
language to use for some of your services, then Twilight is a great choice. If
you have a small bot and just want to get it going in a monolithic application,
then it's also a good choice. It's easy to split off parts of your application
into other services as your application grows.

## Gateway

One of the popular design choices when creating a multi-serviced application is
to have a service that only connects shards to the gateway and sends the events
to a broker to be processed. Twilight is an excellent choice for this use case:
just receive and send the payloads to the appropriate broker stream. Twilight
shards need only partially deserialize payloads to function.

## HTTP proxy ratelimiting

If you have multiple services or lambda functions that can make HTTP requests,
then you'll run into ratelimiting issues. Twilight's HTTP client supports
proxying, and can be combined with something like our very own [http-proxy]
to proxy requests and ratelimit them.

## The sky is the limit

You can do so much more than just this, and that's the beauty of the ecosystem:
it's flexible enough to do anything you need, and if you find something it can't
then we'll fix it. The goal is to remove all limitations on designs and allow
you to do what you need.

[http-proxy]: https://github.com/twilight-rs/http-proxy
