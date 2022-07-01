# 今日学习汇总
## 交叉编译
交叉编译：编译器运行平台和目标可执行文件运行平台不同时需要使用到的技术
rust中解决方案还蛮多的
如`build`的时候增加`--target`选项(通过`rustup`添加相关的平台)
而这里通过`.cargo`目录下配置`config`来达到类似的目的（在`Makefile`中`env`处检查安装了目标平台）
```
[build]
target = "riscv64gc-unknown-none-elf"

[target.riscv64gc-unknown-none-elf]
rustflags = [
    "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes"
]

```

## rust代码
相关的知识点会对应到代码中以注释的方式说明
