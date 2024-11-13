# lamu

The minimal runtime for AWS Lambda to run any arbitrary executable files.

```console
RUSTFLAGS="-C target-cpu=neoverse-n1" cross build --target aarch64-unknown-linux-gnu --release
```

```
RUSTFLAGS="-C target-cpu=neoverse-n1 -C opt-level=z -C strip=symbols" cross build --target aarch64-unknown-linux-gnu --release
```

Ref:
- https://github.com/johnthagen/min-sized-rust

## Terraform example

This is the example to define lambda layer (common base layer that includes `lamu` as bootstrap runtime), and run a shell script as lambda function handler.

Download binary from [Releases](https://github.com/kyontan/lamu/releases) and save as `bootstrap`

```console
$ curl -L https://github.com/kyontan/lamu/releases/latest/download/lamu -O bootstrap
```

```hcl
data "archive_file" "lamu" {
  type        = "zip"
  output_path = "${path.root}/.archive_files/lamu.zip"

  source_file = "bootstrap"
}

resource "aws_lambda_layer_version" "lamu" {
  description         = "lamu"
  filename            = data.archive_file.lamu.output_path
  layer_name          = "lamu-bootstrap-layer"
  compatible_runtimes = ["provided.al2023"]
  source_code_hash    = data.archive_file.lamu.output_base64sha256
}
```

Let's define lambda function just returning "hello", and use the lambda layer defined above.

```
data "archive_file" "hello" {
  type        = "zip"
  output_path = "${path.root}/.archive_files/hello.zip"

  source {
    filename = "handler"
    content  = <<-EOF
    #!/bin/sh

    echo "hello"
    EOF
  }
}

# setup the actual lambda resource
resource "aws_lambda_function" "hello" {
  lifecycle {
    ignore_changes = [
      filename,
      last_modified,
    ]
  }

  function_name = "hello"
  role          = aws_iam_role.lambda.arn
  handler       = "function.handler"
  runtime       = "provided.al2023"
  architectures = ["arm64"]
  timeout       = 1
  memory_size   = 128

  filename         = data.archive_file.hello.output_path
  source_code_hash = data.archive_file.hello.output_base64sha256
  layers = [
    aws_lambda_layer_version.lamu.arn,
  ]
}

resource "aws_iam_role" "lambda" {
  name = "lambda-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Principal = {
          Service = "lambda.amazonaws.com",
        },
        Action = "sts:AssumeRole",
      },
    ],
  })
}

resource "aws_iam_role_policy_attachment" "lambda_basic_execution" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = aws_iam_role.lambda.name
}
```

You can now call function named `hello`, and see the shell script can be used as a Lambda function, with small latency overhead!
