## captcha_rust

 **captcha_rust** is a library that generates picture verification codes. Example pictures are as follows:

![1.png](https://github.com/duzhaosongyue/captcha_rust/blob/main/img/1.png)  ![2.png](https://github.com/duzhaosongyue/captcha_rust/blob/main/img/2.png)
![3.png](https://github.com/duzhaosongyue/captcha_rust/blob/main/img/3.png)   
![4.png](https://github.com/duzhaosongyue/captcha_rust/blob/main/img/4.png)
![5.png](https://github.com/duzhaosongyue/captcha_rust/blob/main/img/5.png) ![6.png](https://github.com/duzhaosongyue/captcha_rust/blob/main/img/6.png)

## Example

Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
captcha_rust =  "0.1.1"
```

And then get started in your `main.rs`:

```rust
use captcha_rust::Captcha;

fn main() {
    let a = Captcha::new(5,130,40);
    println!("test:{},base_img:{}", a.text, a.base_img);
}
```