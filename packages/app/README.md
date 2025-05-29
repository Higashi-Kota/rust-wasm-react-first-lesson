# App - Docking UI Demo Application

`@internal/dock` ライブラリのデモンストレーションアプリケーション

## 🎯 目的

- ドッキング UI ライブラリの動作デモ
- 実際の使用例の提供
- 開発時の動作確認環境

## 🚀 起動方法

```bash
# 開発モード
pnpm dev

# 本番ビルド
pnpm build

# プレビュー
pnpm preview
```

## 🏗️ 構成

- **React 18** - UI ライブラリ
- **TypeScript** - 型安全性
- **Vite** - ビルドツール
- **Tailwind CSS** - スタイリング
- **@internal/dock** - ドッキング UI

## 🔧 カスタマイズ

`src/App.tsx` でコンテンツの追加・変更が可能：

```tsx
import { DockingUIDemo, CounterPanelContent } from "@internal/dock";

function App() {
  return (
    <DockingUIDemo
      availableContents={[
        CounterPanelContent,
        // 独自コンテンツを追加
      ]}
    />
  );
}
```
