# MacroquadとRapierの簡単なサンプル

Rustに慣れるために次はゲームエンジンでも触るかとMacroquadを試してみた。
（Bevyも気になったけど覚えること多くてRust書くまでの距離が長そうだったのでMacroquad を選択）

Macroquadは物理エンジンを持たないのだけど、Rapierという品質の良さそうな汎用のエンジンがあった。
Macroquadとの組み合わせについての記事もあったので、それを参考に実装。

<https://rodneylab.com/rapier-physics-with-macroquad/>

作ったのはよくあるクリックしたところにボールが追加されて下に落ちるだけのもの。
WASM用のビルドも問題なく通ったので以下で動きます。

<https://tnantoka.github.io/hello-macroquad-rapier>

ソースコードはこちら。

<https://github.com/tnantoka/hello-macroquad-rapier>
