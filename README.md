# rust-wasm-react-first-lesson

# Rust WASM × React スターターキット

現代的なWebアプリケーション開発のための、RustのWebAssembly（WASM）とReact+TypeScriptを組み合わせたモノレポ構成のスターターキットです。

## 🚀 特徴

- **🦀 Rust + WebAssembly**: 高性能な計算処理をブラウザで実行
- **⚛️ React + TypeScript**: モダンなフロントエンド開発
- **📦 モノレポ構成**: PNPM Workspace + Cargo Workspace
- **🔄 ホットリロード**: Rust側とReact側の両方でリアルタイム開発
- **🧪 包括的テスト**: Rust単体テスト + React コンポーネントテスト
- **⚡ 高速ビルド**: Vite + wasm-pack による最適化

## 📁 プロジェクト構成

```
TBD
```


---

## 🛠️ セットアップ

### 前提条件

以下のツールをインストールしてください：

- **Node.js** (v22+)
- **pnpm** (v10+)
- **Rust** (最新安定版)
- **wasm-pack** (WASM ビルドツール)

```bash
# Node.js & pnpm
npm install -g pnpm

# Rust (初回のみ)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# cargo-watch (ファイル監視用)
cargo install cargo-watch
```

### 初回セットアップ

```bash
# リポジトリをクローン
git clone <repository-url>
cd rust-wasm-react-starter

# 🚀 ワンコマンドセットアップ
pnpm run setup
```

---

## 🦀 Rust（WASM）側の開発

### 開発コマンド

```bash
# WASM クレートのビルド（開発用）
pnpm run build:wasm
# または
make build-wasm-dev

# WASM クレートの変更監視（自動リビルド）
pnpm run watch:wasm
# または
make watch-wasm

# 本番用 WASM ビルド
pnpm run build:wasm:prod
# または
make build-wasm-prod
```

### テスト

```bash
# 全 Rust クレートのテスト実行
pnpm run test:rust
# または
cargo test --workspace

# 特定のクレートのテスト
cargo test -p wasm-math
cargo test -p wasm-text
cargo test -p wasm-utils

# テストの詳細表示
cargo test --workspace -- --nocapture
```

### コード品質チェック

```bash
# Rust リンティング（Clippy）
pnpm run lint:rust
# または
cargo clippy --workspace --all-targets -- -D warnings

# Rust フォーマット
pnpm run format:rust
# または
cargo fmt --all

# Rust フォーマットチェック
cargo fmt --all -- --check
```

### 個別クレートの操作

```bash
# 特定のクレートをビルド
wasm-pack build packages/crates/wasm-math --target web --scope internal --dev

# 特定のクレートの依存関係チェック
cargo check -p wasm-math

# 特定のクレートのドキュメント生成
cargo doc -p wasm-math --open
```

---

## ⚛️ React（フロントエンド）側の開発

### 開発コマンド

```bash
# React 開発サーバー起動
pnpm run dev
# または
pnpm --filter app dev

# 開発サーバー（ポート指定）
pnpm --filter app dev -- --port 3000
```

### ビルド

```bash
# React アプリの本番ビルド
pnpm --filter app build

# ビルド結果のプレビュー
pnpm --filter app preview
```

### テスト

```bash
# React コンポーネントテスト
pnpm run test:app
# または
pnpm --filter app test

# テスト UI モード
pnpm run test:ui
# または
pnpm --filter app test:ui

# テストカバレッジ
pnpm run test:coverage
# または
pnpm --filter app test:coverage

# CI用テスト（一回実行）
pnpm --filter app test:run
```

### コード品質チェック

```bash
# TypeScript 型チェック
pnpm --filter app typecheck

# ESLint/Biome チェック
pnpm --filter app check

# ESLint/Biome 自動修正
pnpm --filter app check:fix

# コードフォーマット
pnpm --filter app format
```

---

## 🔄 統合開発（推奨）

### 並列開発

```bash
# 🌟 WASM監視 + React開発サーバー（推奨）
pnpm run dev:all

# Utils + WASM監視 + React開発サーバー（フル開発）
pnpm run dev:watch
```

### 統合ビルド

```bash
# 🚀 フル本番ビルド（WASM → Utils → React）
pnpm run build

# 統合テスト（Rust + React 全テスト）
pnpm run test
```

### プロジェクト管理

```bash
# 全パッケージの依存関係チェック
pnpm run package:check
# または
pnpm -r --parallel package:check

# 全パッケージのフォーマット
pnpm run format
# または
pnpm -r --parallel format

# 全パッケージの型チェック
pnpm run typecheck
# または
pnpm -r --parallel typecheck
```

---

## 🧪 テスト戦略

### テストの種類

1. **Rust 単体テスト**: 各 WASM クレートのロジック検証
2. **React コンポーネントテスト**: UI コンポーネントの動作検証
3. **統合テスト**: WASM関数とReactコンポーネントの連携検証

### テスト実行パターン

```bash
# 🔍 開発中の継続テスト
pnpm --filter app test        # React テスト（監視モード）
cargo test --workspace        # Rust テスト（一回実行）

# 🚀 CI/CD用の全テスト
pnpm run test                 # 全テスト（一回実行）

# 📊 詳細テスト
pnpm run test:coverage        # カバレッジ付きテスト
pnpm run test:ui              # ビジュアルテストUI
```

---

## 🛠️ よく使うコマンド集

### 日常開発

```bash
# 朝の開発開始
pnpm run dev:all              # WASM監視 + React サーバー

# 新機能開発
pnpm run watch:wasm           # WASM変更監視（別ターミナル）
pnpm run dev                  # React開発サーバー（別ターミナル）

# テスト駆動開発
pnpm --filter app test        # React テスト監視
cargo test --workspace        # Rust テスト実行
```

### トラブルシューティング

```bash
# 🔄 完全リセット
pnpm run reset                # 全削除 + 再インストール

# 🧹 部分クリーン
pnpm run clean:wasm           # WASM ビルド成果物削除
pnpm run clean:build          # ビルド成果物削除
pnpm run clean:modules        # node_modules 削除

# 🔍 デバッグ
pnpm list --depth=0           # ワークスペースパッケージ確認
cargo check --workspace       # Rust 依存関係チェック
```

### パフォーマンス最適化

```bash
# 本番用最適化ビルド
pnpm run build:wasm:prod      # WASM リリースビルド
pnpm --filter app build       # React 本番ビルド

# ビルドサイズ分析
pnpm --filter app build -- --analyze
```

---

## 📚 開発ワークフロー

### 1. 新機能開発の流れ

```bash
# 1. WASM側の機能実装
cd packages/crates/wasm-math
# Rust コード編集

# 2. WASM テスト
cargo test -p wasm-math

# 3. WASM ビルド
pnpm run build:wasm

# 4. React側の実装
cd packages/app
# TypeScript コード編集

# 5. React テスト
pnpm test

# 6. 統合テスト
pnpm run test
```

### 2. 本番デプロイ準備

```bash
# 1. 全テスト実行
pnpm run test

# 2. コード品質チェック
pnpm run lint:rust
pnpm run check

# 3. 本番ビルド
pnpm run build

# 4. ビルド成果物確認
ls packages/app/dist/
```

---

## 🚨 トラブルシューティング

### よくある問題

#### 1. WASM初期化エラー

```bash
# 解決方法
rm -rf packages/crates/*/pkg
pnpm run build:wasm
pnpm install
```

#### 2. 型定義が認識されない

```bash
# 解決方法
pnpm --filter app typecheck
# VSCode: Cmd/Ctrl + Shift + P → "TypeScript: Restart TS Server"
```

#### 3. ホットリロードが効かない

```bash
# 解決方法
pnpm run dev:all    # 並列監視モード使用
# または別ターミナルで
pnpm run watch:wasm # WASM監視
pnpm run dev        # React開発サーバー
```

#### 4. 依存関係エラー

```bash
# 解決方法
pnpm run reset      # 完全リセット
```

---

## 🎯 推奨開発環境

### VSCode拡張機能

- **Rust Analyzer**: Rust開発サポート
- **Even Better TOML**: Cargo.toml編集
- **ES7+ React/Redux/React-Native snippets**: React開発
- **TypeScript Importer**: 自動インポート
- **Tailwind CSS IntelliSense**: TailwindCSS支援

### 設定例

```json
// .vscode/settings.json
{
  "rust-analyzer.cargo.features": "all",
  "typescript.preferences.includePackageJsonAutoImports": "auto",
  "editor.formatOnSave": true
}
```

---

## 📖 参考資料

- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)
- [PNPM Workspace](https://pnpm.io/workspaces)

---

## 🤝 貢献

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

---

## 📝 ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は [LICENSE](LICENSE) ファイルを参照してください。

---

## 🙏 謝辞

このプロジェクトは以下のオープンソースプロジェクトに基づいています：

- [Rust](https://www.rust-lang.org/)
- [WebAssembly](https://webassembly.org/)
- [React](https://react.dev/)
- [Vite](https://vitejs.dev/)

---

**Happy Coding! 🦀⚛️🚀**