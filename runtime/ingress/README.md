# Ingress for Webhooks

The `automatons-aws-ingress` crate provides a webserver that processes webhooks
from third-party integrations. The webserver verifies the authenticity of the
webhook events, deserializes their payload, and then pushes the events into a
queue for asynchronous processing.

## Features

- Health check to monitor the service health at `/_health`

## Development

`automatons-aws-ingress` is a simple HTTP server written in Rust. It is tightly
integration with AWS, though, so setting up a local development environment
requires a few extra steps.

- Install [Localstack](https://localstack.cloud) for a local cloud environment
- Set up [`awslocal`] to interact with the local cloud environment

With both tools set up, run the following command to start `localstack`:

```shell
localstack start
```

In another terminal session, create a SQS queue:

```shell
awslocal sqs create-queue --region eu-central-1 --queue-name automatons-event-queue
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`awslocal`]: https://docs.localstack.cloud/integrations/aws-cli/#localstack-aws-cli-awslocal
