#!/bin/bash
# クリップボードからパスを読み取る
path=$(pbpaste)
# abc何番かを抽出する
delimiter="/"
abc_no=$(echo $path | cut -d $delimiter -f1)


# target_path="~/dev/atcoder-rust/$abc_no/"
# eval target_path=$target_path
# echo $target_path
# cd $target_path

echo hello

# パスからファイル名を抽出する
filename=$(basename "$path" .rs)
# echo $filename

# cargo compete testコマンドにファイル名を渡す
cargo compete test "$filename"