# physics-sandbox

Rust、Bevy、そして Avian (`avian2d`) を使用したシンプルな2D物理シミュレーションの練習用プロジェクトです。

## 技術スタック
- **Language**: Rust
- **Game Engine**: Bevy
- **Physics Engine**: Avian (`avian2d`)

## 特徴
- 2Dの剛体シミュレーション（静的な床と、重力で落下してバウンドする動的なボール）
- 物理デバッグ描画プラグイン (`PhysicsDebugPlugin`) によるコライダー（当たり判定）の可視化
- 反発係数（`Restitution`）や摩擦（`Friction`）の調整設定

## 実行方法
プロジェクトのルートディレクトリで以下のコマンドを実行します：

```bash
cargo run
```

## 主なコードのポイント (`src/main.rs`)
- **物理プラグインの導入**: `PhysicsPlugins` と `PhysicsDebugPlugin`
- **重力設定**: `Gravity` リソースの定義
- **オブジェクト生成**: 
  - 床：`RigidBody::Static` と `Collider::rectangle`
  - ボール：`RigidBody::Dynamic` と `Collider::circle`、反発・摩擦の調整