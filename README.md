# 概要

連分数展開を適当な長さ計算させたい．

```
cargo run 3.1415926525
```
のように実行すると、`3.1415926525`の部分を`f64`にparseして、その（正則）連分数展開を計算する。 
`f64`どうしの普通の四則計算で素直な求め方を実装して、長さ200以上になるか、次の分母とそれを越えない最大の整数との差が`0.00000000000001`より小さくなるかすると打ち切るようにした。