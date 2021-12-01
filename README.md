## Example

Add warp and Tokio to your dependencies:

```toml
captcha_rust =  "0.1.0"
```

And then get started in your `main.rs`:

```rust
fn main() {
    let a = Captcha::new(5,130,40);
    println!("test:{},base_img:{}", a.text, a.base_img);
}
```