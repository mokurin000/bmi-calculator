# bmi-calculator

BMI计算器，基于中华人民共和国统计数据。

注意，BMI只是一种指标，不代表体脂率等等。

## 构建

构建之前，请确保已部署[ndless环境](https://lights0123.com/ndless-rust/)

```bash
cargo +nightly ndless build -- --release
```
## 运行

如果您有已正常运行的 Firebird Emu ，您可以运行以下命令：

```bash
cargo +nightly ndless run #调试构建
cargo +nightly ndless run -- --release #发布构建
```
