# Devcontainer Claude Code 開発環境

このリポジトリは、**devcontainer** を使用して **Claude Code** の開発環境を構築する方法を学習するために作成されました。Rust の最小プロジェクトと、VS Code の devcontainer 内でコードを実行するための設定ファイルが含まれています。

## 特徴

- Rust プロジェクトの雛形 (`src/main.rs`)
- Devcontainer 設定 (`.devcontainer/`)
- コンテナ管理スクリプト (`mise-tasks/`)
- プロンプトと設計ドキュメントの例

## はじめに

1. **mise** をインストールします (https://mise.jdx.dev)。
2. `mise run container-up` を実行して devcontainer を起動します。
3. コンテナ内でプロジェクトをビルド・実行できます:
   ```bash
   cargo build
   cargo run
   ```
4. 提供されているスクリプトでワークツリーの管理やコンテナの停止ができます。

## コントリビューション

セットアップを試したり、例を追加したり、ドキュメントを改善したりして、自由に貢献してください。
