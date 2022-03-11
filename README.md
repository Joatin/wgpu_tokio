# WGPU Tokio

This crate provide helper traits for working with [wgpu](https://github.com/gfx-rs/wgpu) in an async fashion. This 
crate depends on tokio for it's async scheduling.

## Why
Using tokio in gamedev brings a lot of benefits. Using multiple threads allows us to utilize the cpu to it's fullest. 
And tokios green thread abstraction is cheap and performant. 

However, straight of calling wgpu's api from the tokio context is not a good idea. Most them will block the code in 
order to do some chit-chatting with the gpu, etc. This will block the current worker thread and block other task from 
executing while we are waiting for wgpu to finnish.

A better solution if to offload this work to a dedicated thread pool and let them run, and then notify tokio once their 
finnished.

This crate does all of this and puts it inside a nice and tidy trait.

## Usage

Make sure you have the dependencies in place

```toml
[dependencies]
wgpu = "0.12"
wgpu_tokio = "0.12"
```

Then in you code make sure you are using the trait

```rust
use wgpu_tokio::DeviceAsyncExt;
```

The trait requires that the device is behind and ```Arc```. Then just go ahead and do something.

```rust
async fn do_something(device: Arc<Device>) {
    let my_buffer = device.create_buffer_async(...).await;
}
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/joatin/wgpu_tokio/blob/main/LICENSE.txt