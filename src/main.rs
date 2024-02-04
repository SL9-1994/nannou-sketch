/// これは、画面内をボールが反射し、波紋を作成するシンプルなNannouアートです。
/// ボールはウィンドウの境界内で水平方向と垂直方向に動きます。
/// ボールがウィンドウの端に達すると、方向と色が変わり、ボールが反射した位置に波紋を発生させます。
/// 波紋は時間とともに拡大し、ある半径に達すると消えます。
/// このスケッチは、NannouライブラリとRandライブラリを使用しています。
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

// 波紋構造体
struct Ripple {
    x: f32,
    y: f32,
    radius: f32,
    alpha: f32,
}

// メイン構造体
struct Model {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: f32,
    ripples: Vec<Ripple>,
}

/// アプリケーションのモデルを初期化
fn model(app: &App) -> Model {
    let x = 100.0;
    let y = 100.0;
    let dx = 2.5;
    let dy = 2.0;

    let color = 1.0;

    let ripples = Vec::new();

    let _window = app.new_window().size(800, 400).view(view).build().unwrap();
    Model {
        x,
        y,
        dx,
        dy,
        color,
        ripples,
    }
}

/// アプリケーションの状態を更新
fn update(app: &App, model: &mut Model, _update: Update) {
    // ボールの位置を更新
    model.x = model.x + model.dx;
    model.y = model.y + model.dy;

    // ウィンドウを取得
    let win_rect = app.window_rect();
    let mut rng = rand::thread_rng();

    // ボールがウィンドウの左右に衝突した際
    if (model.x > win_rect.right() - 35.0) || (model.x < win_rect.left() + 35.0) {
        model.dx = model.dx * -1.0;
        model.color = rng.gen_range(0.0..1.0);
        model.ripples.push(Ripple {
            x: model.x,
            y: model.y,
            radius: 0.0,
            alpha: 5.0,
        });
    }

    // ボールがウィンドウの上下に衝突した際
    if (model.y > win_rect.top() - 35.0) || (model.y < win_rect.bottom() + 35.0) {
        model.dy = model.dy * -1.0;
        model.color = rng.gen_range(0.0..1.0);
        model.ripples.push(Ripple {
            x: model.x,
            y: model.y,
            radius: 0.0,
            alpha: 5.0,
        });
    }

    for ripple in &mut model.ripples {
        ripple.radius += 1.0; // 波紋の半径を拡大
        ripple.alpha -= 0.05; // 不透明度を減少
    }

    // ある半径に達した波紋を削除
    model.ripples.retain(|ripple| ripple.radius < 200.0);
}

/// アプリケーションをレンダリング
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // ボールを描画
    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(70.0, 70.0)
        .color(nannou::color::hsv(model.color, 1.0, 1.0))
        .stroke(BLACK);

    // 波紋を描画
    for ripple in &model.ripples {
        draw.ellipse()
            .no_fill()
            .stroke_weight(2.0)
            .stroke(rgba(0.0, 1.0, 1.0, ripple.alpha))
            .x_y(ripple.x, ripple.y)
            .radius(ripple.radius);
    }

    draw.to_frame(app, &frame).unwrap();
}
