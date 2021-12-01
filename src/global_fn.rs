use image::{ImageBuffer, Rgb};
use imageproc::drawing::{draw_cubic_bezier_curve_mut, draw_hollow_ellipse_mut, draw_text_mut};
use rand::{thread_rng, Rng};
use rusttype::Font;
use crate::basic_data::{BASIC_CHAR, BASIC_COLOR, SCALE, WHITE};
use image::ImageOutputFormat::Png;
use image::DynamicImage;

/***
 * 生成随机数
 * params num - 随机数的最大值
 */
fn get_rnd(num: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..=num)
}

/****
 * 生成验证码字符数组
 * params num - 验证码的位数并且最大不能超过53
 */
pub fn get_captcha(num: usize) -> Vec<String> {
    let mut res = vec![];
    for _ in 0..num {
        let rnd = get_rnd(53);
        res.push(BASIC_CHAR[rnd].to_string())
    }
    res
}

/****
 * 获取颜色
 */
fn get_color() -> Rgb<u8> {
    let rnd = get_rnd(4);
    Rgb(BASIC_COLOR[rnd])
}

/****
 * 产生两个数之间的随机数
 * params min – 最小值
 *        max – 最大值
 * return:随机数
 */
fn get_next(min: f32, max: u32) -> f32 {
    min + get_rnd(max as usize - min as usize) as f32
}

/****
 * 获取字体
 */
fn get_font() -> Font<'static> {
    let font = Vec::from(include_bytes!("../font/arial.ttf") as &[u8]);
    Font::try_from_vec(font).unwrap()
}

/****
 * 获取一个白色背景的图片
 */
fn get_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |_, _| {
        image::Rgb(WHITE)
    })
}

/****
 * 循环在背景图片上写入验证码字符
 * params res    - 需要写入的验证码字符数组
 *        image  - 背景图片
 */
fn cyclic_write_character(res: &[String], image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let c = (image.width() - 10) / res.len() as u32;
    let y = image.height() / 2 - 15;
    for (i, _) in res.iter().enumerate() {
        let text = &res[i];
        draw_text_mut(image, get_color(), 5 + (i as u32 * c), y, SCALE, &get_font(), text);
    }
}

/****
 * 画干扰线
 * params image  - 背景图片
 */
fn draw_interference_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let width = image.width();
    let height = image.height();
    let x1: f32 = 5.0;
    let y1 = get_next(x1, height / 2);

    let x2 = (width - 5) as f32;
    let y2 = get_next((height / 2) as f32, height - 5);

    let ctrl_x = get_next((width / 4) as f32, width / 4 * 3);
    let ctrl_y = get_next(x1, height - 5);

    let ctrl_x2 = get_next((width / 4) as f32, width / 4 * 3);
    let ctrl_y2 = get_next(x1, height - 5);
    //随机画贝塞尔曲线
    draw_cubic_bezier_curve_mut(image, (x1, y1), (x2, y2), (ctrl_x, ctrl_y), (ctrl_x2, ctrl_y2), get_color());
}

/****
 * 画干扰圆
 * params num    - 画圆的数量
 *        image  - 背景图片
 */
fn draw_interference_ellipse(num: usize, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for _ in 0..num {
        let w = (2 + get_rnd(5)) as i32;
        let x = get_rnd((image.width() - 25) as usize) as i32;
        let y = get_rnd((image.height() - 15) as usize) as i32;
        draw_hollow_ellipse_mut(image, (x, y), w, w, get_color());
    }
}

/****
 * 将图片转换成base64字符串
 * parma image - 图片
 */
fn to_base64_str(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
    let base_img = DynamicImage::ImageRgb8(image);
    let mut buf = vec![];
    base_img.write_to(&mut buf, Png).unwrap();
    let res_base64 = base64::encode(&buf);
    format!("data:image/png;base64,{}", res_base64)
}

/****
 * 生成图片验证码
 */
pub fn get_captcha_img(res: Vec<String>, width: u32, height: u32) -> String {
    //创建白色背景图片
    let mut image = get_image(width, height);
    //循环将验证码字符串写入到背景图片中
    cyclic_write_character(&res, &mut image);
    //画干扰线
    draw_interference_line(&mut image);
    //画干扰圆
    draw_interference_ellipse(2, &mut image);
    //转换成base64字符串
    to_base64_str(image)
}