# iterが借用するのを忘れがち

`iter()`はコレクションを借用する。
つまりループ中にそのコレクションを使う（move）することはできない。

例えば、これは…

```
struct Player {
  name: String,
  rank: i32,
}

fn main() {
  let players = vec![
    Player {
      name: "John".to_string(),
      rank: 3,
    },
    Player {
      name: "Jane".to_string(),
      rank: 1,
    },
    Player {
      name: "Doe".to_string(),
      rank: 2,
    },
  ];

  for player in players.iter() {
    print_player(player, players);
  }
}

fn print_player(player: &Player, players: Vec<Player>) {
  println!("{} is ranked {} / {}", player.name, player.rank, players.len());
}
```

こうなる。

```
error[E0505]: cannot move out of `players` because it is borrowed
22 |   for player in players.iter() {
   |                 ------- borrow of `players` occurs here
23 |     print_player(player, players);
   |                          ^^^^^^^ move out of `players` occurs here
```

print_layersで参照を受け取るようにすれば解決する。
そもそもこんな無駄な処理をするなという話だが。

なぜエラーになるかと言うと、借用中は移動できないから。
というわけで、以下のシンプルなコードでも再現する。

```
fn main() {
  let a = vec![1, 2, 3];
  let b = &a;
  print_items(a);
  print_items(*b);
}

fn print_items(items: Vec<i32>) {
  for item in items.iter() {
    println!("{}", item);
  }
}
```

```
2 |   let a = vec![1, 2, 3];
  |       - binding `a` declared here
3 |   let b = &a;
  |           -- borrow of `a` occurs here
4 |   print_items(a);
  |               ^ move out of `a` occurs here
5 |   print_items(*b);
  |               -- borrow later used here
  |
```

これも`print_items`が借用すれば解決する。
ちなみに`print_items(*b);`を消すとRustが賢くて`b`を不要と判断するので、借用が発生せずエラーにならない。
（`b`いらんのでは？という警告は出る）
