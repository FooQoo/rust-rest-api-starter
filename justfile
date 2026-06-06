# このプロジェクトでよく使うコマンドを集めたタスクランナー設定。
# `just <task>` で実行できる。`just` だけ叩くとレシピ一覧が出る。
#
# 公式ドキュメント: https://just.systems

# .env ファイルを読み込んで環境変数として展開する
# (DATABASE_URL などがレシピ内で使えるようになる)
set dotenv-load

# レシピ実行用シェルに ~/.cargo/bin を通す
# just は内部で sh を起動するが、その sh は ~/.zshrc を読まないため
# 明示的に PATH を渡さないと cargo が見つからない (git hook と同じ問題)
export PATH := env_var('HOME') + "/.cargo/bin:" + env_var('PATH')

# ──────────────────────────────────────────────────────────────
# Default
# ──────────────────────────────────────────────────────────────

# 引数なしで実行された時に走るレシピ (一覧表示)
default:
    @just --list

# ──────────────────────────────────────────────────────────────
# 開発
# ──────────────────────────────────────────────────────────────

# サーバーを起動
run:
    cargo run

# `-x run` は `cargo run` を実行する指示 (`-x` = execute)
# `-c` は再ビルド前にターミナルをクリアして見やすくする

# ファイル変更を検知して自動で再ビルド + 再起動 (要 cargo-watch)
dev:
    cargo watch -c -x run

# ビルドのみ (実行はしない)
build:
    cargo build

# リリースビルド (最適化あり、デプロイ用)
build-release:
    cargo build --release

# `:` の後ろに別レシピ名を書くと、その依存レシピが先に実行される。
# 既にビルド済みなら cargo が差分なしと判断して即終了するので無駄はない。

# リリースビルドしたバイナリを直接実行 (本番想定)
start: build-release
    ./target/release/rust-rest-api-starter

# ──────────────────────────────────────────────────────────────
# 品質チェック (pre-commit hook と同じ内容を手動で回せる)
# ──────────────────────────────────────────────────────────────

# rustfmt を適用 (書き換える)
fmt:
    cargo fmt

# フォーマット差分があるかチェック (書き換えない)
fmt-check:
    cargo fmt --check

# clippy を実行 (警告を全部エラー扱いに昇格)
lint:
    cargo clippy --all-targets -- -D warnings

# テスト
test:
    cargo test

# 一括チェック: fmt-check → lint → test (pre-commit と同じ)
check: fmt-check lint test

# ──────────────────────────────────────────────────────────────
# DB
# ──────────────────────────────────────────────────────────────

# SQLite ファイルを削除 (次回 run 時に初期データで再生成される)
db-reset:
    rm -f data/rest_starter.db
    @echo "DB removed. Run 'just run' to recreate."

# ──────────────────────────────────────────────────────────────
# 動作確認用の curl ヘルパー
# サーバーが起動している前提
# ──────────────────────────────────────────────────────────────

# 社員数を取得
curl-count:
    curl -s http://localhost:8080/v1/member/count

# 社員を検索 (例: just curl-search John)
curl-search name:
    curl -s "http://localhost:8080/v1/member/search?name={{name}}"

# ──────────────────────────────────────────────────────────────
# クリーンアップ
# ──────────────────────────────────────────────────────────────

# target/ ディレクトリを削除 (ビルドキャッシュ削除)
clean:
    cargo clean
