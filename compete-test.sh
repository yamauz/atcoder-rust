#!/bin/bash
# クリップボードからパスを読み取る
path=$(pbpaste)

echo $path

# パスからファイル名を抽出する
filename=$(basename "$path" .rs)


# cargo compete testコマンドにファイル名を渡す
cargo compete test "$filename"