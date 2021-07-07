# bug-rusoto-cloudformation-localstack

This illustrates a possible bug in either
[localstack](https://github.com/localstack/localstack)
or
[rusoto](https://github.com/rusoto/rusoto)
involving the api equivalent of
`aws cloudformation describe-stacks`
CLI command.

When Rust code is executed against localstack like this:

```
AWS_ACCESS_KEY_ID=test AWS_SECRET_ACCESS_KEY=test AWS_DEFAULT_REGION=us-east-1 LOCALSTACK_ENDPOINT=http://localhost:4566 cargo t
```

the following error occurs:

```
thread 'tests::describe_stacks' panicked at 'called `Result::unwrap()` on an `Err` value: ParseError("Expected EndElement DescribeStacksResponse got Some(Ok(EndDocument))")', src/lib.rs:30:14
```

Localstack may be launch like this:

```
docker run --rm -it -p 4566:4566 -e LOCALSTACK_API_KEY=xxx localstack/localstack:0.12.4
```

The `-e LOCALSTACK_API_KEY=xxx` is optional. The problem occurs with and without it, and with wide range of
localstack versions.

On the contrary, when executed against real AWS the test passes successfully.
