# PC-sleep-checker
## 概要
Rustで開発したPC利用状況・睡眠時間記録アプリです。

## 使用時状況
<img width="496" height="429" alt="image" src="https://github.com/user-attachments/assets/2e29a15d-93ba-4ddf-8b27-6e08f07c488c" />


## 開発背景
生活リズムやPC利用状況を把握したいと考え、自主的に開発しました。

## 使用技術
- Rust
- egui
- CSV
- chrono

## 機能
- PC操作状況の監視
- スリープ時間の記録
- CSV入出力

## 工夫した点
- Rustを独学しながら開発
- AI活用による開発効率化
- 実行時、記録用のディレクトリがない場合、CSVの記録用ディレクトリを生成
- CSVの記録にて先月以前の記録は削除される点

## 動かし方
対象のディレクトリ内にて  
cargo build  
cargo run

## これから足したいこと
昼休みなど定められた休憩時間の間は記録をとらないように設定する機能
