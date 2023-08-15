# clean-architecture-playground

クリーンアーキテクチャーに法って TODO アプリを作成したい

## セットアップ

#### 前提

- sqlx-cli
- direnv

#### データベース

```bash
$ sqlx database create
$ sqlx migrate run
```

### git hooks

```bash
$ git config --local core.hooksPath .githooks
```
