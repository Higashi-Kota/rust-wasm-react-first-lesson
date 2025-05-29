現代のRustによるWASM開発におけるベストプラクティスに基づき、以下の要件をすべて満たす **初心者向けのプロジェクトキッティング手順** を、**ステップバイステップかつ日本語で** 提案してください。上からなぞったら完成形にたどり着けるように再現性を意識した手番を組んでください。

---

### 🧭 目的とスコープ

* フロントエンドに **React + TypeScript + TailwindCSS** を使用し、WASMで構築されたRustの処理と連携する簡単なWebアプリ（例：ハローワールド、四則演算）を作成する。
* WASM側はRust製で、一部の処理（ロジック）をRustに移譲。
* 初心者がRust×WASM×React開発に取り組む際の「ハンズオンデモ」として活用できること。

---

### 🧱 モノレポ構成（pnpm workspace + cargo workspace）

* モノレポ構成は2段構成とする：

  * **pnpm workspace** によるフロントエンド管理
  * **cargo workspace** によるRustクレート管理
* 構成は以下のGitHubレポをベースとし、それぞれの設定を反映：

  * [docking-manager-keep-state](https://github.com/Higashi-Kota/docking-manager-keep-state)

    * pnpm, Vite, Biome, React, Vitest, TailwindCSS などの導入方法、およびルート package.json の scripts を忠実に再現。
  * [rust-backend-api-first-lesson](https://github.com/Higashi-Kota/rust-backend-api-first-lesson)

    * cargo workspace の記述方法とパッケージ分割の構成を参考にRust側を整備。

---

### 🧩 Rust (WASM) 側の構成（複数クレートのcargo workspace）

Rust/WASM側は以下のような**3つのクレートを含むcargo workspace**としてください：

#### A. wasm-math

* add(a, b), sub(a, b), mul(a, b), div(a, b) などの四則演算。
* #[wasm_bindgen] でJavaScriptと接続。

#### B. wasm-text

* reverse(s: &str) -> String（文字列の反転）
* count_vowels(s: &str) -> usize（母音の数を数える）

#### C. wasm-utils

* random_between(min: i32, max: i32) -> i32（乱数生成）
* is_even(n: i32) -> bool（偶数判定）

> すべてのクレートは wasm-pack build を用いて --target web オプションでビルドし、パッケージとしてJS側から利用可能にすること。

---

### 🖥️ フロントエンド側の構成（React + Vite）

* packages/frontend に以下を導入：

  * Vite
  * React（+フック）
  * TypeScript
  * TailwindCSS
  * Biome
  * Vitest（テスト）

* WASMパッケージは、ESM形式でモノレポ構成でインポート：

  
ts
  import init, { add, reverse } from '@xxx/wasm-math';
  await init();
  const result = add(1, 2);


---

### 🔁 ホットリロードと開発体験の最適化

* wasm-pack による --watch ビルド機能を使用し、Rustのコード変更を即時ビルド。
* Viteでは、WASM出力ディレクトリを server.watch 設定で監視し、ブラウザを自動リロード。
* RustとReactの**両方に対応したホットリロード開発環境**を構築。

---

### 🧪 テストと実践ユースケース

#### A. **Vitestによるフロントエンドテスト**

* WASM関数を呼び出すReactコンポーネント（例：Calculator）のテスト。

  * ユニットテスト：add の返り値の検証。
  * 結合テスト：ユーザー操作と表示が正しく連動しているか。
  * モックテスト：WASM関数をmockし、JSロジックのみテスト。

#### B. **Rust側ユニットテスト**

* 各クレートに #[cfg(test)] でRust内のロジックを検証。

  * エッジケースや失敗パスも網羅。

#### C. **バインディング例**

* Rust側の関数を #[wasm_bindgen] で公開し、JS（TypeScript）から呼び出し。
* wasm-bindgen を使った文字列処理、数値処理、構造体の受け渡しの簡単な例を提示。

---

### 📦 成果物として含めるべきもの

* 各セットアップ・操作手順の**日本語による丁寧な説明**
* 実行コマンド（例：pnpm install, wasm-pack build, pnpm devなど）
* 初心者がつまずきやすいポイントとその対処法
* Rust <-> TypeScript 間の型バインディングの実例と注意点
* vite.config.ts や tailwind.config.ts 等の設定ファイルテンプレート

---

この条件に基づき、**初心者が安心してWASM×Reactのモダン開発環境に入門できるハンズオン形式のガイド**を、日本語で提示してください。

また、ディレクトリ構成については以下に従ってください。

# Rust WASMクレートのモノレポ構成と開発手順

## ディレクトリ構成の最適化案

新しいモノレポ構成では、Rust製WASMクレートとフロントエンド（React+Vite）を一つのリポジトリで管理します。`packages/`ディレクトリ以下にサブパッケージを配置し、Rustクレート群は`packages/crates/`配下にまとめます。Reactアプリは`packages/app/`に配置し、PNPMワークスペースで管理します。全体の構成例を以下に示します:

```plaintext
project-root/
├── Cargo.toml                   # Cargoワークスペース定義（Rust全クレート）
├── rust-toolchain.toml          # Rustツールチェインバージョン指定
├── pnpm-workspace.yaml          # PNPMワークスペース定義（packages配下を登録）
├── Makefile                     # 開発・ビルド用コマンド定義
└── packages/
    ├── crates/
    │   ├── wasm-math/
    │   │   ├── Cargo.toml       # Rustクレート定義（WASM出力用設定を含む）
    │   │   ├── src/lib.rs      # クレート実装（wasm_bindgenでエクスポート）
    │   │   └── pkg/            # wasm-packビルド出力（JS/WASMとpackage.json）
    │   ├── wasm-text/
    │   │   └── ...             # （wasm-textクレートのCargo.toml, src, pkg/ 等）
    │   └── wasm-utils/
    │       └── ...             # （wasm-utilsクレートのCargo.toml, src, pkg/ 等）
    └── app/
        ├── package.json        # フロントエンドReactアプリ（依存関係にWASMパッケージを含む）
        ├── vite.config.ts      # Vite設定（WASM対応の調整を含む）
        ├── tsconfig.json       # TypeScript設定
        └── src/                # Reactアプリのソースコード
```

このようにRustと前端を一元管理することで、依存関係の共有や開発効率の向上が期待できます。特にCargoワークスペースによるRustクレートの統合管理と、PNPMワークスペースによるパッケージ間リンクを組み合わせることで、モノレポ内でのスムーズなモジュール参照が可能になります。

## Cargoワークスペースの設定（Rust側）

Rust側では、リポジトリ直下の`Cargo.toml`でワークスペースを定義し、WASMクレートをメンバーに含めます。例えば以下のように記述します:

```toml
[workspace]
members = [
    "packages/crates/wasm-math",
    "packages/crates/wasm-text",
    "packages/crates/wasm-utils",
    # 他にバックエンドクレート等があればここに追加
]
```

これにより、各クレートはワークスペース共通のRustツールチェインや依存を利用できます（参考:では`task-backend`等複数クレートをworkspace.membersに指定）。各WASMクレートの`Cargo.toml`側では、WASM向けビルドのため**crateタイプをcdylib**に設定します。例えば`packages/crates/wasm-math/Cargo.toml`に以下を追加します:

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

* `"cdylib"`はWASMの生成に必要な設定で、wasm32ターゲット用のバイナリを生成します。`rlib`も指定しておくと、Cargo経由で通常のRustライブラリとしてもビルドでき、必要なら単体テストなども可能になります。
* 各クレートのソース（`src/lib.rs`）には、`wasm-bindgen`クレートを使って関数や値を`#[wasm_bindgen]`でエクスポートしてください。例えば、`add`関数を公開する場合は`#[wasm_bindgen] pub fn add(x: i32, y: i32) -> i32 { x + y }`のように記述します。これにより、ビルド後のJSから`add`関数を呼び出せるようになります。

Cargoワークスペース構成にすることで、`cargo build --workspace`や`cargo test --workspace`で複数クレートをまとめて操作できる利点があります。また、依存クレートのバージョンを一括管理できるため、ビルド時間の短縮やコード共有が容易になります。

## PNPMワークスペースと名前空間付きインポート設定

フロントエンド（React側）はPNPMワークスペースで管理します。`pnpm-workspace.yaml`には、`packages/app`およびWASMクレートの出力先である`packages/crates/*/pkg`ディレクトリを含めます。これにより、それら`pkg`フォルダが**ローカルなnpmパッケージ**として認識されます。例えば、pnpm-workspace.yamlに次のエントリを追加します（既存のエントリに続けて）:

```yaml
packages:
  - "packages/app"
  - "packages/crates/*/pkg"
```

各WASMクレートはビルド後に`pkg/`ディレクトリ内に`package.json`が生成されます。**重要なのは`package.json`の名前にスコープ付きパッケージ名を設定すること**です。`wasm-pack build`実行時に`--scope`オプションを指定すると、自動生成される`package.json`の"name"にスコープが付与されます（例: crate名が`wasm-math`でスコープ`@xxx`の場合、`name: "@xxx/wasm-math"`となる）。以下のコマンド例は、crate `wasm-math`をビルドしスコープを適用するものです:

```bash
wasm-pack build packages/crates/wasm-math --target web --scope xxx --dev
```

> **補足:** `--target web`を指定するとブラウザで直接動作するESM形式のモジュールを出力します（Webpackなどのバンドラが不要な形式）。`--dev`は開発ビルド用で、高速なビルドとデバッグ用情報を有効にします。本番用ビルド時は`--release`を指定してください。

上記コマンドにより、`packages/crates/wasm-math/pkg/`に以下のようなファイル群が生成されます：

* `package.json` – `@xxx/wasm-math`という名前やバージョン、モジュールエントリなどを含む
* `wasm_math_bg.wasm` – コンパイルされたWASMバイナリ
* `wasm_math.js` – WASM呼び出し用のJavaScriptラッパー（ESMモジュール）
* `wasm_math.d.ts` – 型定義ファイル（TypeScript用）

同様に`wasm-text`や`wasm-utils`についても`wasm-pack build --scope xxx`を実行し、それぞれ`@xxx/wasm-text`等の名前でパッケージ生成します。こうして得られた各`pkg`フォルダが、PNPMワークスペース内のローカル依存パッケージになります。**フロントエンドからそれらを名前空間付きでインポートできるようになる設定例**として、`packages/app/package.json`のdependenciesに以下のような参照を追加します:

```json
{
  "dependencies": {
    "@xxx/wasm-math": "0.1.0",
    "@xxx/wasm-text": "0.1.0",
    "@xxx/wasm-utils": "0.1.0",
    "...": "..." 
  }
}
```

ここではバージョン`0.1.0`を例示していますが、これは各WASMクレートのCargo.tomlで定義したバージョンと合わせておくとよいでしょう（wasm-packが生成するpackage.jsonにも同じバージョンが入ります）。PNPMではworkspace内のパッケージ同士で**バージョンが一致**すれば自動的にシンボリックリンクで解決されます。例えば上記のように依存関係に追加しておけば、`pnpm install`時に`@xxx/wasm-math`がローカルの`packages/crates/wasm-math/pkg`から供給されます。これにより、フロントのコード中で `import { add } from '@xxx/wasm-math';` のように書けば、ビルド済みWASMパッケージをESMとして利用可能になります。外部のnpmレジストリに公開しなくても、モノレポ内で名前空間付きパッケージとして参照できるのがポイントです。

> **補足:** スコープ名`@xxx`はプロジェクトに合わせて設定してください。社内パッケージの場合`@internal`のようなスコープを使うこともできます（例: docking-managerプロジェクトでは`@internal/dock`といったパッケージ名を使用）。

## wasm-packビルドとVite／TypeScript設定の調整

**wasm-packによるビルド:** 各WASMクレートについて、開発時には`wasm-pack build --target web --scope xxx --dev`を実行します（前述の通り）。これを手動で行う代わりに、スクリプトやMakefileで自動化することも検討できます。例えばMakefileにターゲットを用意して`make build-wasm`で全クレートのビルドをまとめて実行できるようにするか、package.jsonのスクリプトに`"build:wasm": "...複数のwasm-packコマンド..."`を記述する方法があります。

**Vite側の調整:** ViteはデフォルトでESMのWASMモジュールを扱うことができます。`wasm-pack`出力のJSは、WASMファイルを動的にロードするコードを含んでいます。通常`--target web`の出力では、`fetch`や`WebAssembly.instantiateStreaming`を使ってWASMを読み込む実装になっており、ブラウザ上で動的に`*.wasm`ファイルを取得します。その際、**ビルド時にはVite（Rollup）がWASMファイルをアセットとして認識**し、`dist`にコピーしてくれます。Viteは`import.meta.url`を使ったWASM参照も適切に処理するため、通常追加のプラグイン無しで動作します。ただし、確実に機能させるため以下の点を確認・調整してください:

* **Viteの別名設定（alias）:** もしフロントエンドコード内で`@xxx/wasm-math`を直接インポートしてうまく解決されない場合、`vite.config.ts`で別名を設定できます。ただし、PNPMワークスペースで依存解決できていれば基本不要です。Viteは`node_modules`内のパッケージとして解釈するため、`@xxx/wasm-math`をそのまま認識するはずです。特殊なディレクトリ構造で問題が出る場合のみ、例えば以下のようにalias指定を検討してください（通常不要）:

  ```ts
  // vite.config.ts の例
  import { defineConfig } from 'vite';
  import path from 'path';
  export default defineConfig({
    resolve: {
      alias: {
        '@xxx/wasm-math': path.resolve(__dirname, '../crates/wasm-math/pkg')
      }
    },
    // ...他の設定
  });
  ```

  上記のaliasはビルド前の`pkg`フォルダを直接参照しています。基本的には、wasm-pack実行後にpnpm経由でパッケージ解決する方が簡潔なので、alias設定は必要ないでしょう。

* **WASMファイルの配置:** 開発時はVite開発サーバーが`@xxx/wasm-math`内の`*.wasm`ファイルを提供できるようになります。ビルド時（本番ビルド時）にはRollupが`wasm_math_bg.wasm`（または`*_bg.wasm`）を出力ディレクトリにコピーし、JSコード中の参照をそのパスに書き換えます。特殊な設定は不要ですが、万一ビルド成果物にWASMが含まれない場合は、Viteの設定で`publicDir`に手動でコピーするか、`vite-plugin-wasm`の導入を検討します。一般的には、WASMを動的インポートするコードがあれば自動で含まれます。

* **TypeScriptサポート:** wasm-packが生成する`*.d.ts`型定義のおかげで、TypeScriptのプロジェクトでもWASM関数を型安全に利用できます。`@xxx/wasm-math`パッケージ内のd.tsファイルは自動的に拾われますが、TSconfigで特別な設定をする必要は基本ありません。`tsconfig.json`で`"moduleResolution": "node"`を指定し、`node_modules`内の型定義を参照できる状態であればOKです（一般的にデフォルトで問題ありません）。もし型が認識されない場合、`typeRoots`に`node_modules/@types`以外のパスを追加する方法もありますが、このケースでは不要でしょう。

* **ESMモジュールとしての利用:** フロントエンドコードでは、WASMパッケージを**非同期初期化**する必要があります。たとえば、生成されたJSには既定で`init`という初期化関数（またはデフォルトエクスポート）が含まれていることがあります。Reactコンポーネントで利用する際は、アプリ起動時に一度`await initWasm()`（例: `await init();`）を実行しWASMを読み込んでから、`add`などのエクスポート関数を呼び出すように実装します。こうすることで、WASMモジュールの準備ができ次第、計算ロジック（例: `add(2,3)`）をJavaScript側で利用できます。初期化関数名や呼び出し方法は生成された`README.md`や`package.json`にも記載がありますので、各パッケージごとに確認してください。

## Makefileおよびビルドスクリプトの修正案

既存のMakefileがある場合、パスやスクリプトをモノレポ構成に合わせて更新します。参考リポジトリでは、ワークスペース化に伴いMakefileやDockerfileのパスをルートに移動し、内容を修正しています。本プロジェクトでも以下のポイントを検討してください:

* **wasm-packビルドのターゲット追加:** MakefileにWASMクレート用のビルドコマンドを用意します。例えば:

  ```makefile
  build-wasm:
      wasm-pack build packages/crates/wasm-math --target web --scope xxx --dev
      wasm-pack build packages/crates/wasm-text --target web --scope xxx --dev
      wasm-pack build packages/crates/wasm-utils --target web --scope xxx --dev
  ```

  などとし、`make build-wasm`で全WASMクレートをビルドできるようにします。本番ビルド用に`--release`版を実行するターゲットも用意すると良いでしょう。

* **開発用Watchコマンド:** 開発効率を上げるため、WASMクレートのソースコード変更を監視して自動ビルドする仕組みを整えます。Makefileで`wasm-pack watch`相当の処理を行うには、`cargo-watch`を利用すると便利です。例として:

  ```makefile
  watch-wasm:
      cargo watch -w packages/crates/wasm-math/src -x 'run-wasm-math-build' &
      cargo watch -w packages/crates/wasm-text/src -x 'run-wasm-text-build' &
      cargo watch -w packages/crates/wasm-utils/src -x 'run-wasm-utils-build'
  ```

  （※ここで`run-wasm-math-build`はwasm-packビルドを呼ぶカスタムコマンド名とします）のようにして、各ディレクトリを監視しつつ並列でビルドすることが考えられます。ただし、簡単なアプローチとしては**ターミナルを分けて手動でwatchを実行**する方法でも十分です。たとえば「ターミナル1で`cargo watch -w packages/crates/wasm-math -s "wasm-pack build ..."`を実行、ターミナル2でVite開発サーバーを起動」のように並行実行しても良いでしょう。

* **フロントエンドビルドとの連携:** `pnpm build`コマンドにWASMビルドを組み込むことも検討します。ルートの`package.json`にスクリプトを追加し、`"build": "pnpm run build:wasm && pnpm -r --filter app build"`のように記述すれば、まずWASMクレート群をビルドしてからReactアプリのビルドを行えます（docking-managerの例ではライブラリ→アプリの順にビルドしている）。このように順序を制御することで、アプリのビルド時に最新のWASM資産が含まれるようになります。

* **その他パス修正:** Docker関連のファイルやCI設定において、パスが変更になった場合（例えば`packages/`配下への移動）、適宜パスを書き換えます。また環境変数ファイル(.env)の場所が変わる場合は、それを読み込むスクリプトやREADMEの指示も更新してください。

## 開発フローとホットリロードの検証

**並列開発サーバー起動:** 新構成では、Rust側（WASMクレート群）とフロントエンド側を並行して開発できます。推奨される開発環境として、**PNPMのマルチ実行機能**や`concurrently`を用いて「WASMクレートのビルド監視」と「Reactアプリのホットリロードサーバー起動」を同時に実行します。例えば、ルートのpackage.jsonに以下のようなスクリプトを追加します:

```json
{
  "scripts": {
    "dev:wasm": "pnpm run build-wasm --watch", 
    "dev:app": "pnpm --filter app dev", 
    "dev:all": "concurrently \"pnpm run dev:wasm\" \"pnpm run dev:app\""
  }
}
```

上記は概念例ですが、`pnpm run dev:all`と実行するとWASMクレート群のウォッチビルドとReactの開発サーバーを**並列起動**できます（実際の実装では`concurrently`パッケージの導入が必要です）。あるいは、PNPM標準の`-r --parallel`オプションを使い、各パッケージ（appや必要ならクレートのpkgにスクリプト定義）に対して並行実行する方法もあります。

**ホットリロードの挙動:**

* *Reactコードの変更:* 従来通りViteがファイル変更を検知し、HMR（Hot Module Replacement）でコンポーネントを差し替えます。開発者の体験としては即座にブラウザに変更が反映される状態が維持されます。
* *Rust(WASM)コードの変更:* `cargo watch`や上記`dev:wasm`スクリプトにより、自動で再コンパイル→`pkg/`内のWASMバイナリとJSが更新されます。Vite開発サーバーはデフォルトでは`node_modules`内の変更を監視しませんが、**PNPMワークスペースでのローカルパッケージの場合シンボリックリンク経由で変更を検知できる場合があります**。念のため、Viteの開発サーバー設定で監視パスに`packages/crates/**/pkg`を含める設定を追加してもよいでしょう（例: `server.watch`オプションでパターン指定）。こうしておくと、WASMパッケージの再ビルド完了時にViteがそのモジュールの更新を検知し、自動的にブラウザをリロードします。結果として、Rust側のコード修正→保存でブラウザがリフレッシュされ、最新のWASMロジックが読み込まれる流れが実現します。

完全なHMR（状態を保持したままモジュール差し替え）という点では、WASMモジュールはページリロードが発生するためReactのコンポーネント状態などはリセットされます。しかし、開発効率という観点ではRust→WASMの変更を即座に試せるメリットがあります。**Docking Manager**の例でも、ライブラリ（今回はWASMクレートに相当）とアプリを並行起動しホットリロード対応する運用を推奨しています。同様に、本構成でもRustとReact双方の変更がスムーズに反映される開発体験を維持できます。

**検証ポイント:** 実際に再構成後の環境で以下を確認してください：

* `pnpm install`で依存関係をインストール後、`pnpm run build:wasm`（または初回のみ`make build-wasm`等）を実行し、各`pkg/`に正しくビルド成果物が出力されているか。特に`package.json`の名前（@スコープ付き）が想定通りか確認します。
* Reactアプリ(`packages/app`)の`package.json`にWASMパッケージが追加されているか、`pnpm install`後に`node_modules/@xxx/wasm-math`へのシンボリックリンクが張られているか確認します。
* `pnpm run dev:all`を起動し、ブラウザでアプリを開いた状態で、WASMクレート内のRustコードを変更→保存します。自動で`wasm-pack build`が走り、少し待つとブラウザがリロードされ、新しいロジックが反映されるか検証します。例えば`add`関数の戻り値を変えてみて、JS側で呼んだ結果が変われば成功です。
* また、ReactのUIコードを変更した際に従来通りHMRでUIが更新されることも確認してください。WASMの組み込みによって通常のフロントエンド開発体験が損なわれていないことが重要です。

以上を踏まえ、再構成された開発フローの一例をまとめると次のようになります:

1. **初回セットアップ:** リポジトリをクローン後、`pnpm install`で依存をインストールします。Rustツールチェインもプロジェクト推奨バージョン（`rust-toolchain.toml`参照）を用意してください。必要に応じてデータベースやバックエンドの環境も起動します。
2. **WASMパッケージのビルド:** 初回のみ各WASMクレートをビルドします（`pnpm run build:wasm`または`make build-wasm`など）。これにより`pkg/`が生成され、フロントエンドからインポート可能になります。
3. **開発サーバー起動:** `pnpm run dev:all`を実行し、WASMクレートのウォッチビルドとReactアプリのViteサーバーを同時に起動します。コンソール上で両方のログを確認しつつ、ブラウザで`http://localhost:3000`（または設定したポート）にアクセスします。
4. **開発・改修:** 以降はRust側のコードを編集すれば自動ビルドが走り、ブラウザがリロードされます。フロント側のコード修正は即座にHMRで反映されます。両者の修正を繰り返しつつ開発を進めます。
5. **ビルドとデプロイ:** 機能が完成したら、`pnpm build`を実行して本番ビルドを行います（内部でWASMクレートを`--release`ビルドするようスクリプトを組んでおく）。`packages/app/dist/`以下に出力された静的ファイル一式（JS/CSS/WASMなど）をサーバーやホスティングサービスに配置すれば、ブラウザ上でWASMを活用するReactアプリが動作します。

このような手順でモノレポを構成すれば、RustとReactの開発を統合しつつ効率的に進めることができます。参考リポジトリで実現していたCargoワークスペース管理やホットリロード開発も維持され、**並列実行による快適な開発体験**が得られるでしょう。各設定ファイルの微調整（パスの修正や依存関係の定義など）は必要ですが、上記の構成案に沿って実装することで要件を満たす環境を構築できるはずです。

**Sources:**

* Higashi-Kota/rust-backend-api-first-lesson（Cargoワークスペース構成の参考）
* Higashi-Kota/docking-manager-keep-state（PNPMモノレポ構成とスクリプトの参考）
* Rust Wasm-Pack Official Docs（wasm-packによるスコープ付きパッケージ名の設定）
