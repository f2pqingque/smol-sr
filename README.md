# smol-sr
minimal sr server aimed for minimal file size.

it uses the `smol` runtime instead of the usual go-to `tokio`.

it can still do normal, "not hardcoded", battle from json. (plan)

sdkserver is abusing `build.rs`, can probably go lower if using `hyper` instead too.

## binary sizes
- sdkserver: 305 KB (312,320 bytes)
- gameserver: 276 KB (283,136 bytes)

![image](https://github.com/user-attachments/assets/3ed60175-19df-451e-be23-0b82b7860b1a)

## tutorial:
```
cargo run --release --bin sdk
cargo run --release --bin gme
```
