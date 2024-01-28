

## Running geckodriver

```bash
geckodriver -p 4444
```

```rust

use thirtyfour::prelude::*;

let caps = DesiredCapabilities::firefox();
// NOTE: this assumes you have a WebDriver compatible server running
//       at http://localhost:4444
//       e.g. `geckodriver -p 4444`
let driver = WebDriver::new("http://localhost:4444", caps).await?;
driver.goto("https://www.rust-lang.org/").await?;
// Always remember to close the session.
driver.quit().await?;

```

## Kill the port

```bash
fuser -n tcp -k 4444
```
