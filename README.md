# daemon_agent
the daemon for agent application.

## Usage
```shell
# build release
cargo build --release

# got bin
target/release/daemon
target/release/app_a
target/release/app_b

# run
nohup target/release/daemon target/release/app_a target/release/app_b > /dev/null 2>&1 &

```
## TodoList
### kill daemon
When daemon has been killed, app_a and app_b keep running.
