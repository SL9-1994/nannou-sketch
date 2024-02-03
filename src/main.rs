// Nannouのアイテムを一括インポート。
use nannou::prelude::*;

/// アプリケーションのメインモデルを表します。
struct Model {}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

/// モデルを初期化します。
///
/// # Arguments
///
/// * `_app` - `App` 構造体への参照。
///
/// # Returns
///
/// 初期化された `Model` 構造体。
fn model(_app: &App) -> Model {
    Model {}
}

/// イベントがトリガーされたときにモデルを更新します。(イベントハンドラ)
///
/// # Arguments
///
/// * `_app` - `App` 構造体への参照。
/// * `_model` - `Model` 構造体への参照。
/// * `_event` - トリガーされたイベント。
fn event(_app: &App, _model: &mut Model, _event: Event) {}

/// アプリケーションの描画処理を行います。
///
/// # Arguments
///
/// * `_app` - `App` 構造体への参照。
/// * `_model` - `Model` 構造体への参照。
/// * `_frame` - ビューをレンダリングするフレーム。
fn view(_app: &App, _model: &Model, _frame: Frame) {}
