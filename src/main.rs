use macroquad::prelude::*;

#[macroquad::main("millie1")]
async fn main() {
  let hero_set = Texture2D::from_file_with_format(include_bytes!("../assets/hero.png"), None);
  //let hero_set = load_texture("assets/hero.png").await.unwrap();
  let (mut w, mut h) = (48., 64. * 2.);
  let (mut x, mut y) = (screen_width() / 2., screen_height() / 2.);
  let mut shift_pressed;
  loop {
    let hero_source: Rect = Rect::new(w, h, 48., 64.);
    clear_background(BLACK);
    draw_texture_ex(
      hero_set,
      x,
      y,
      WHITE,
      DrawTextureParams {
        source: Some(hero_source),
        ..Default::default()
      },
    );
    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
      shift_pressed = 2.;
    } else {
      shift_pressed = 1.;
    }
    if w < 48. * 2. {
      w += 48.;
    } else {
      w = 0.
    }
    if is_key_down(KeyCode::W) {
      y -= 5. * shift_pressed;
      h = 0.;
    } else if is_key_down(KeyCode::A) {
      x -= 5. * shift_pressed;
      h = 64. * 3.;
    } else if is_key_down(KeyCode::S) {
      y += 5. * shift_pressed;
      h = 64. * 2.;
    } else if is_key_down(KeyCode::D) {
      x += 5. * shift_pressed;
      h = 64. * 1.;
    } else {
      h = 64. * 2.;
      w = 48. * 1.
    }
    next_frame().await;
    std::thread::sleep(std::time::Duration::from_millis(50));
  }
}
