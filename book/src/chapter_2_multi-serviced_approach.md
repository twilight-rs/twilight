# Services

Twilight is built with a service-minded approach. This means that it caters to
both monolithic and multi-serviced applications equally. If you have a very
large bot and have a multi-serviced application and feel like Rust is a good
language to use for some of your services, then Twilight is a great choice. If
you have a small bot and just want to get it going in a monolithic application,
then it's also a good choice. It's easy to split off parts of your application
into other services as your application grows.

## Gateway clusters

One of the popular design choices when creating a multi-serviced application is
to have a service that simply connects shards to the gateway and sends the
events to a broker to be processed. As bots grow into hundreds or thousands of
shards, multiple instances of the application can be created and clusters -
groups of shards - can be managed by each. Twilight is a good choice for this
use case: you can receive either events that come in in a loop and send the
payloads to the appropriate broker stream, or you can loop over received
payloads' bytes to send off.

## Gateway session ratelimiting

If you have multiple clusters, then you need to queue and ratelimit your
initialized sessions. The Gateway includes a Queue trait which you can
implement, and the gateway will submit a request to the queue before starting a
session. Twilight comes with a queue that supports sharding and Large Bot
sharding, but when you start to have multiple clusters you'll want to implement
your own. Our [gateway-queue] is an example of this.

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

[gateway-queue]: https://github.com/twilight-rs/gateway-queue
[http-proxy]: https://github.com/twilight-rs/http-proxy
