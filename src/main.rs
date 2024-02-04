// // Nannouのアイテムを一括インポート。
// use nannou::prelude::*;

// /// アプリケーションのメインモデルを表します。
// struct Model {}

// fn main() {
//     nannou::app(model).event(event).simple_window(view).run();
// }

// /// モデルを初期化します。
// ///
// /// # Arguments
// ///
// /// * `_app` - `App` 構造体への参照。
// ///
// /// # Returns
// ///
// /// 初期化された `Model` 構造体。
// fn model(_app: &App) -> Model {
//     Model {}
// }

// /// イベントがトリガーされたときにモデルを更新します。(イベントハンドラ)
// ///
// /// # Arguments
// ///
// /// * `_app` - `App` 構造体への参照。
// /// * `_model` - `Model` 構造体への参照。
// /// * `_event` - トリガーされたイベント。
// fn event(_app: &App, _model: &mut Model, _event: Event) {}

// /// アプリケーションの描画処理を行います。
// ///
// /// # Arguments
// ///
// /// * `_app` - `App` 構造体への参照。
// /// * `_model` - `Model` 構造体への参照。
// /// * `_frame` - ビューをレンダリングするフレーム。
// fn view(app: &App, _model: &Model, frame: Frame) {
//     // キャンバスを取得
//     let draw = app.draw();

//     // キャンバスの背景色を設定
//     draw.background().color(GRAY);

//     // 1辺が200の正方形を原点を中心として表示
//     // draw.rect().x_y(0.0, 0.0).w_h(200.0, 200.0).color(ORANGE);

//     // Windowの幅を取得
//     let boundary = app.window_rect();

//     // 正弦波を作成
//     let sine = app.time.sin();

//     // 速度を1/2にした正弦波を作成
//     let slowersine = (app.time / 2.0).sin();

//     // 正弦波をWindowのx,yの最大値・最小値にマッピングする
//     let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
//     let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

//     // 正円を作成
//     draw.ellipse().w_h(x / 2.0, y / 2.0).color(WHITE).x_y(x, y);

//     // フレームにレンダリング
//     draw.to_frame(app, &frame).unwrap();
// }

// 画像描画・テクスチャの作成
use nannou::prelude::*;

struct Model {
    // テクスチャ構造体を追加
    texture: wgpu::Texture,
}

fn main() {
    // アプリケーションを実行
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    // 512*512のウィンドウを作成
    app.new_window().size(512, 512).view(view).build().unwrap();

    // nannouにassetsを読み込む
    let assets = app.assets_path().unwrap();

    // 画像のパスを読み込む
    let img_path = assets.join("images").join("universe").join("quasar1.jpg");

    // 指定したパスから、テクスチャを読み込む
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    // textureフィールドを持つModelインスタンスを作成
    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // フレームを初期化
    frame.clear(BLACK);

    // キャンバスを取得
    let draw = app.draw();

    // テクスチャを描画
    draw.texture(&model.texture);

    // フレームにレンダリング
    draw.to_frame(app, &frame).unwrap();
}
