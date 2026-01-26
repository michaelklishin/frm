# rabbitmq-versioning

RabbitMQ version parsing, comparison, and URL generation.

## Features

- Parse RabbitMQ version strings (e.g., `4.2.3`, `v4.2.3`, `4.3.0-alpha.1`)
- Support for prerelease versions: alpha, beta, and rc
- Version comparison following semantic versioning rules
- Generate download URLs for RabbitMQ generic Unix builds

## Usage

```rust
use rabbitmq_versioning::{Version, Prerelease};

// Parse versions
let v: Version = "4.2.3".parse().unwrap();
let alpha: Version = "4.3.0-alpha.1".parse().unwrap();

// Check version type
assert!(v.is_ga());
assert!(alpha.is_alpha());

// Compare versions
assert!(v < alpha.base_version());

// Generate URLs
let url = v.download_url();
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
