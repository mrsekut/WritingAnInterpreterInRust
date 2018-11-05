# 概要

- ソースコード　 → 　トークン列　 → 　抽象構文木

## 字句解析

- 字句解析器 Lexer

### token のデータ構造

- type
- ファイル名
- 行番号

## 構文解析

## 抽象構文木(AST)

## 内部オブジェクトシステム

## 評価器

# Rust

### use

- `use self::module_a` `self`はディレクトリの`.`
- `use super::module_a` `super`はディレクトリの`..`
- 参考 https://keens.github.io/blog/2017/01/15/rustnomoju_runokirikata/

## 疑問

- usize, char ってなんぞ ★
- struct の部分初期化は無理なのか
- derive[Debug]とは
- なぜ同じ構造体の impl の中で他のメソッドを使えないのか
  - no method named 'hoge' froun for type '' in the current scope
- impl fn の (&mut self)とは
- キャスト

## 参考

- モジュールシステム
  - https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
