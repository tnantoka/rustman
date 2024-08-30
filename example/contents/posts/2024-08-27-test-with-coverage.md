# テスト（カバレッジ付き）

<https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#from-prebuilt-binaries>

READMEに従ってインストールする。

```
host=$(rustc -vV | grep '^host:' | cut -d' ' -f2)
curl --proto '=https' --tlsv1.2 -fsSL https://github.com/taiki-e/cargo-llvm-cov/releases/latest/download/cargo-llvm-cov-$host.tar.gz | tar xzf - -C "$HOME/.cargo/bin"
```

後は以下を実行すればカバレッジを出力しつつテストが実行される。

```
$ cargo llvm-cov --open
```

特に躓くところはなかった。
setupとかteardownがないようで、ちょっと困ったけど。
