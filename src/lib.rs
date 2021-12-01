mod basic_data;
mod global_fn;

use global_fn::{get_captcha, get_captcha_img};

#[test]
fn main() {
    let a = Captcha::new(5,130,40);
    println!("test:{},base_img:{}", a.text, a.base_img);
}


struct Captcha {
    pub text: String,
    pub base_img: String,
}

impl Captcha {
    pub fn new(num: usize, width: u32, height: u32) -> Self {
        //生成验证码字符数组
        let res = get_captcha(num);
        let text = res.join("");
        //根据验证码字符数组生成图片并且把图片转化成base64字符串
        let base_img = get_captcha_img(res, width, height);
        Captcha {
            text,
            base_img,
        }
    }
}