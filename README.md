## captcha_rust

 **captcha_rust** is a library that generates picture verification codes. Example pictures are as follows:  

 ![1.png](https://github.com/duzhaosongyue/img/blob/main/1.png?raw=true) | ![2.png](https://github.com/duzhaosongyue/img/blob/main/2.png?raw=true) | ![3.png](https://github.com/duzhaosongyue/img/blob/main/3.png?raw=true) 
   ---- | ----- | ------  
 ![4.png](https://github.com/duzhaosongyue/img/blob/main/4.png?raw=true) | ![5.png](https://github.com/duzhaosongyue/img/blob/main/5.png?raw=true) | ![6.png](https://github.com/duzhaosongyue/img/blob/main/6.png?raw=true)

## Example

Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
captcha_rust =  "0.1.3"
```

And then get started in your `main.rs`:

```rust
use captcha_rust::Captcha;

fn main() {
    let a = Captcha::new(5,130,40);
    println!("test:{},base_img:{}", a.text, a.base_img);
}
```