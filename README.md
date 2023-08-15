# clean-architecture-playground

[![CI](https://github.com/BaseW/clean-architecture-playground/actions/workflows/ci.yml/badge.svg?event=pull_request)](https://github.com/BaseW/clean-architecture-playground/actions/workflows/ci.yml)

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
