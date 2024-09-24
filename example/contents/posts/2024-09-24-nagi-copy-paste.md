# Nagi（WRY製テキストエディタアプリ）でコピペのキーボードショートカットをサポート

自分で使っていて、⌘+Cなどが動かないことに気づいた。
（右クリックのメニューでしか使えない）

WRYやTaoは`for Tauri`とちゃんと書いてあるとおりTauriのために作られたCrateで、丁寧なドキュメントはないのでしばらく彷徨った。

以前はTaoでできたようだが、今はmudaというCrateに分けられていた。
<https://github.com/tauri-apps/tao/commit/d0b20c94eaf555ba27f3cfbbf2636e3f3b036a97>

mudaのexamplesにTaoとの連携例があった。
<https://github.com/tauri-apps/muda/blob/07c42f8f6f8e8dcf8128f3c6e4d52a3c3e99dd71/examples/tao.rs>

この通りにやったら無事動いたのでv0.0.2をリリースした。

<https://github.com/tnantoka/nagi/releases/tag/v0.0.2>

Menuのアプリ名がCargo.tomlのpackage nameを参照するようで、ここを変更しないと大文字始まりにできなかったのは少しハマった。
