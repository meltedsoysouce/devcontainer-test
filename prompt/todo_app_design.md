# CLI Todo管理アプリケーション 実装計画書

## 概要
Rustで実装するシンプルなCLIベースのTodo管理アプリケーション。
単一ファイル（main.rs）で完結し、最小限の依存関係で動作する設計。

## アーキテクチャ

### 設計方針
- **単一責任**: 各関数は明確な単一の責任を持つ
- **エラー処理**: Result型を使用した明示的なエラーハンドリング
- **永続化**: JSONファイルによるシンプルな永続化
- **ユーザビリティ**: 直感的なコマンドインターフェース

## データ構造

### Todo構造体
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: usize,
    description: String,
    completed: bool,
    created_at: String,
}
```

### TodoList構造体
```rust
#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}
```

### Command列挙型
```rust
enum Command {
    Add(String),
    List,
    Done(usize),
    Delete(usize),
    Help,
    Exit,
    Unknown,
}
```

## コマンドインターフェース

| コマンド | 説明 | 使用例 |
|---------|------|--------|
| `add <description>` | 新しいタスクを追加 | `add 買い物リストを作成` |
| `list` | 全タスクを表示 | `list` |
| `done <id>` | タスクを完了としてマーク | `done 1` |
| `delete <id>` | タスクを削除 | `delete 2` |
| `help` | ヘルプメッセージを表示 | `help` |
| `exit` | アプリケーションを終了 | `exit` |

## 関数シグネチャと責務

### メイン関数
```rust
fn main() -> Result<(), Box<dyn std::error::Error>>
```
- アプリケーションのエントリーポイント
- 初期化処理とメインループの管理

### コマンド解析
```rust
fn parse_command(input: &str) -> Command
```
- ユーザー入力を解析してCommand列挙型に変換
- 入力検証とコマンド分類

### Todo操作関数

#### 追加
```rust
fn add_todo(list: &mut TodoList, description: String)
```
- 新しいTodoをリストに追加
- 自動的にIDを割り当て、タイムスタンプを付与

#### 一覧表示
```rust
fn list_todos(list: &TodoList)
```
- 全Todoをフォーマットして表示
- 完了/未完了の状態を視覚的に表現

#### 完了マーク
```rust
fn complete_todo(list: &mut TodoList, id: usize) -> Result<(), String>
```
- 指定IDのTodoを完了状態に変更
- 存在しないIDの場合はエラーを返す

#### 削除
```rust
fn delete_todo(list: &mut TodoList, id: usize) -> Result<(), String>
```
- 指定IDのTodoをリストから削除
- 存在しないIDの場合はエラーを返す

### 永続化関数

#### 保存
```rust
fn save_to_file(list: &TodoList, path: &str) -> Result<(), Box<dyn std::error::Error>>
```
- TodoListをJSON形式でファイルに保存
- ホームディレクトリの`.todos.json`に保存

#### 読み込み
```rust
fn load_from_file(path: &str) -> Result<TodoList, Box<dyn std::error::Error>>
```
- JSONファイルからTodoListを復元
- ファイルが存在しない場合は空のリストを返す

### ユーティリティ関数

#### ヘルプ表示
```rust
fn print_help()
```
- 使用可能なコマンドと使い方を表示

#### ファイルパス取得
```rust
fn get_data_file_path() -> String
```
- OSに応じた適切なデータファイルパスを返す
- デフォルト: `~/.todos.json`

## 依存関係（Cargo.toml）

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "2.0"
chrono = "0.4"
```

## 実装フロー

### 1. 初期化フェーズ
1. データファイルパスの決定
2. 既存データの読み込み（存在する場合）
3. TodoList構造体の初期化

### 2. メインループ
```rust
loop {
    // プロンプト表示
    print!("> ");
    
    // 入力受付
    let input = read_line();
    
    // コマンド解析
    let command = parse_command(&input);
    
    // コマンド実行
    match command {
        Command::Add(desc) => {
            add_todo(&mut list, desc);
            save_to_file(&list, &path)?;
        }
        Command::List => list_todos(&list),
        Command::Done(id) => {
            complete_todo(&mut list, id)?;
            save_to_file(&list, &path)?;
        }
        Command::Delete(id) => {
            delete_todo(&mut list, id)?;
            save_to_file(&list, &path)?;
        }
        Command::Help => print_help(),
        Command::Exit => break,
        Command::Unknown => println!("Unknown command. Type 'help' for usage."),
    }
}
```

### 3. エラーハンドリング
- ファイルI/Oエラー: 適切なエラーメッセージを表示し、処理を継続
- 不正なID: ユーザーにフィードバックを提供
- JSON解析エラー: デフォルト値で初期化

## 表示フォーマット

### Todo一覧表示例
```
=== TODO LIST ===
[1] ✓ 買い物リストを作成 (2024-01-20 10:30)
[2] ○ レポートを書く (2024-01-20 11:00)
[3] ○ 会議の準備 (2024-01-20 14:15)
================
Total: 3 | Completed: 1 | Pending: 2
```

### カラー仕様
- 完了タスク: 緑色
- 未完了タスク: 黄色
- エラーメッセージ: 赤色
- 成功メッセージ: 青色

## テスト戦略

### ユニットテスト対象
1. `parse_command`: 各種入力パターンのテスト
2. `add_todo`: Todo追加とID割り当ての確認
3. `complete_todo`: 完了状態の変更確認
4. `delete_todo`: 削除処理とリストの整合性確認

### 統合テスト
1. ファイル保存・読み込みのラウンドトリップ
2. 複数コマンドの連続実行
3. エラー状態からの復帰

## セキュリティ考慮事項
- ファイルパスのサニタイゼーション
- 入力長の制限（説明文は最大500文字）
- ファイル権限の適切な設定（600）

## パフォーマンス最適化
- メモリ効率: Vec<Todo>の事前容量確保
- ファイルI/O: 変更時のみ保存
- JSON: 最小限のフィールドのみシリアライズ

## 拡張可能性
将来的な機能追加を考慮した設計：
- タグ/カテゴリー機能
- 期限設定
- 優先度管理
- 検索/フィルター機能
- エクスポート機能（CSV, Markdown）

## 実装優先順位
1. **Phase 1**: 基本CRUD機能
   - add, list, delete コマンド
   - メモリ上でのデータ管理

2. **Phase 2**: 永続化
   - JSONファイルへの保存/読み込み
   - エラーハンドリング

3. **Phase 3**: UX向上
   - カラー出力
   - ヘルプメッセージ
   - 入力検証の強化

## コード品質基準
- Rustの慣習に従ったコーディング
- `clippy`による静的解析をパス
- `rustfmt`によるフォーマット
- 適切なドキュメントコメント
- エラーメッセージの明確性