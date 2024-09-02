# cargo-bundleでMacアプリを作る

[前回WRYでHello Worldを作った](/posts/2024-08-31-hello-wry)ので、今回はそれをMacアプリにしてみる。

`cargo-bundle`を使えば簡単だった。

```
$ cargo install cargo-bundle
$ cargo bundle --release
$ open target/release/bundle/osx/hello-wry.app
```

これだけでOK。
 
アプリ名やアイコンなどはCargo.tomlで設定できる。何も無しでも一応動いた。

設定例 
<https://github.com/burtonageo/cargo-bundle?tab=readme-ov-file#example-cargotoml>

この方法でWRY製のアプリを配布できそう。
