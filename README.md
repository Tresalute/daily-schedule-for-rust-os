# 2022年夏日Rust Os 日常学习计划

- labs: 对比原项目的代码，加入一些自己的理解和想法
- learing-notes: 正常的学习笔记，对这一天学习的知识的汇总与回顾
- pratice: rust的一些小demo,练练手


*七月*

| Mon               | Tues              | Wed                          | Thur                         | Fri                          | Sat               | Sun               |
| ----------------- | ----------------- | ---------------------------- | ---------------------------- | ---------------------------- | ----------------- | ----------------- |
|                   |                   |                   |                   | 1 <br> ([D1](#day-1-202271)) | 2 <br> ([D2](#day-2-202272)) | 3 <br> ([D3](#day-3-202273)) | 
|4 <br> ([D4](##day-4-202274)) | 5 <br> ([D5](#day-5-202275)) | 6 <br> ([D6](#day-6-202276)) | 7 <br> ([D7](#day-7-202277)) | 8 <br> ([D8](#day-8-202278))       | 9 <br> ([D9](#day-9-202279))            | 10 <br> ([D10](#day-10-2022710))         | 
|11  <br>  ([D11](#day-11-2022711))             | 12      <br>    ([D12](#day-12-2022712))       | 13    <br>    ([D13](#day-13-2022713))             | 14         <br>    ([D14](#day-14-2020711))        | 15        <br>    ([D15](#day-15-2022715))                    | 16    <br>     ([D16](#day-16-2022716))                       | 17    <br>      ([D17](#day-17-2022717))                       |
|18    <br>    ([D18](#day-18-2020718))            | 19   <br>     ([D19](#day-19-2022719))            | 20   <br>    ([D20](#day-20-2022720))            | 21       <br>    ([D21](#day-21-2022721))         | 22     <br>    ([D22](#day-22-2022722))                         | 23     <br>    ([D23](#day-23-2022723))                         | 24    <br>    ([D24](#day-24-2022724))                        | 
|25      <br>    ([D25](#day-25-2022725))             | 26         <br>    ([D26](#day-26-2022726))           | 27         <br>    ([D27](#day-27-2022727))           | 28       <br>    ([D28](#day-28-2022728))           | 29         <br>    ([D29](#day-29-2022729))                    | 30        <br>    ([D30](#day-30-2022730))                     | 31     <br>    ([D31](#day-31-2022731))                           |


## Day 1 2022/7/1
今天本是在那儿摸鱼，逛着逛着在知乎看到了这个项目，恰逢最近有想了解下操作系统底层相关知识的想法，只能说都是缘分，遂入之。

前段时间恰逢有rust的项目，刚好学了点，所以基础语法这块暂时可以略过，等后面遇见不懂的再弄。

说说今天的学习情况吧
1. 使用codespace配置了环境
2. 跑了下os1
3. 参照着yunwei37师兄的日记和文档详细的看了下os1的代码，收获满满

计划下明天
1. 遵循着文档敲一遍os1,毕竟实践才是硬道理
2. 学习下RISC-V相关内容，这块没撒经验，估计着要头大
3. 复习下rust,写一些小demo之类的

## Day 2 2022/7/2
虽然预料到会完成不了任务，但没想到这么惨

今天的学习情况
1. 在本地搭建了下学习环境，codespace还是有很多不知之处啊。
2. 照着文档敲了一遍os1,中途磕磕碰碰的，填了很多坑，但还是有很多不懂的地方

明天
1. 继续剖析os1的代码
2. 看看RISC-V ([OpenSBI](https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#legacy-sbi-extension-extension-ids-0x00-through-0x0f))

睡了睡了，明天不出去浪了，专心学习

## Day 3 2022/7/3
头大头大，太多知识盲点，英文还差，啃RISC-V的文档真的太痛苦了

今天
1. 完善了os1的相关注释，知识点一个个敲出来，但还是感觉很朦胧
2. 啃了下RISC-V的文档，英文硬伤，不过多多少少了解一点了，后续当字典查吧
3. 了解了RUST中内联汇编相关的知识，顺带拓展了RISC-V的汇编


明天
1. 继续了解下RISV-V的相关知识，可以的话用其写写demo
2. rsut相关demo也得补充下，算法之类的

*时间全靠上班摸鱼来，到了考验我的摸鱼技术*

## Day 4 2022/7/4
有一说一，今天没摸到什么鱼，回家也颓废了，💊</br>
不过今天写rust的时候遇到一个问题，到现在也没什么特别好的解决方法</br>
说下一吧，不然总感觉今天撒也没做(～￣▽￣)～</br>

关于Rust所有权的问题</br>
在类中自调用该类自己的方法时，提示所有权问题，说借用重复</br>
当时卡在这儿半天，后面分析，原因是函数闭包中重复应用了参数操作,具体记不清了，明天去公司补充吧</br>

还有说准备用汇编写demo的，没写出来，看了一小会，头大，明天看看怎么学这玩意(ノへ￣、)

---
更新下遇到的代码
```rust

use std::collections::HashMap;

struct Example{
            sp_map:HashMap<u8, u8>,
            bp_map:HashMap<u8, String>,
        }

        impl Example{
            pub fn get_dp_map(&mut self, _key: &u8) {
                todo!()
            }

            pub fn get_sp_data(&mut self, key: &u8){
                match self.sp_map.get(key) {
                    Some(v) => {
                        self.get_dp_map(v);
                    },
                    None => {},
                }
            }
        }
/// 报错
/// cannot borrow `*self` as mutable because it is also borrowed as immutable
/// mutable borrow occurs hererustcE0502
/// example.rs(286, 23): immutable borrow occurs here
/// example.rs(288, 30): immutable borrow later used by call


```
解释下在闭包中变量`v`被二次使用就不行了，如果通过clone能避开这个问题，但治标不治本(#｀-_ゝ-)

---
刚又研究了下，其实错误提示已经给出来了...</br>
在闭包中，`self`的权限是借用还是引用由上层调用者决定，这里是`self.sp_map.get()`</br>
它返回的是一个值的引用，所以这里`self`的权限是引用，而在闭包中的调用`get_dp_map`中对`self`进行了借用</br>
导致了这个问题

---
修改后的代码
```rust
use std::collections::HashMap;

struct Example{
            sp_map:HashMap<u8, u8>,
            bp_map:HashMap<u8, String>,
        }

        impl Example{
            pub fn get_dp_map(& self, _key: &u8) {
                todo!()
            }

            pub fn get_sp_data(&mut self, key: &u8){
                match self.sp_map.get(key) {
                    Some(v) => {
                        self.get_dp_map(v);
                    },
                    None => {},
                }
            }
        }
```

## Day 4 2022/7/4
更新更新</br>
今天工作虽然没摸鱼但回家同样也没摸鱼，除了学这门课程外还有另外的东西要搞，现在又是半夜了，好饿(#｀-_ゝ-)</br>

说下今天
1. 看了下RISC-V的部分汇编指令集，相对Inter/AMD x86&64真的简单好多，老师写的demo基本都能看懂了，自己写吗...还得等会(～￣▽￣)～
2. 对着指导书看了下os2,然后看了看了源码，跑了跑了，现在还剩`__restore`那儿没整明白，明天摸鱼瞅瞅（~~如果能成功摸到鱼(ಥ _ ಥ)~~）

说下明天
1. 继续今天未完成的任务
2. 按着指导书谢谢,注视下自己的理解
3. 学习笔记还是空着的,找个时间记录下吧,暂定周末吧

溜ε=( o｀ω′)ノ


