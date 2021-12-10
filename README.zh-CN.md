<!--
 * @Author       : sunzhifeng <ian.sun@auodigitech.com>
 * @Date         : 2021-08-25 21:35:39
 * @LastEditors  : sunzhifeng <ian.sun@auodigitech.com>
 * @LastEditTime : 2021-12-10 22:27:37
 * @FilePath     : /iceoryx-rs/README.zh-CN.md
 * @Description  : Created by sunzhifeng, Please coding something here
-->

# 产品介绍

## Mac 构建说明

```bash
  https://github.com/elBoberido/iceoryx-rs.git
  git clone https://github.com/elBoberido/iceoryx-rs.git --recursive
  ls
  cd iceoryx-rs
  ls
  cd iceoryx
  cd ../
  code ./
  cd "/Users/ian/GitHub/refs/iceoryx-rs"

  # 所在路径：/Users/ian/GitHub/refs/iceoryx-rs/iceoryx
  cd iceoryx
  ICEORYX_DIR=$PWD
  mkdir -p build

  # 所在路径：/Users/ian/GitHub/refs/iceoryx-rs/iceoryx/build
  cd build
  git clone https://github.com/mirror/ncurses.git
  cd ncurses
  git checkout v6.2
  ./configure  --prefix=$ICEORYX_DIR/build/dependencies/ --exec-prefix=$ICEORYX_DIR/build/dependencies/ --with-termlib
  make -j12
  make install
  cd ../../
  # 所在路径：/Users/ian/GitHub/refs/iceoryx-rs/iceoryx
  cmake -Bbuild -Hiceoryx_meta -DCMAKE_PREFIX_PATH=$(PWD)/build/dependencies/
  cmake --build build -j 4
  cd ../


  # 所在路径：/Users/ian/GitHub/refs/iceoryx-rs
  # 构建 debug 版本
  cargo build --all --examples

  # 构建 release 版本
  cargo build --all --release --examples
```

## 运行 Demo

Open three terminals. 开启 3 个终端窗口

- start RouDi `target/iceoryx-install/bin/iox-roudi`
- start the publisher `target/debug/examples/publisher_simple`
- start a subscriber `target/debug/examples/subscriber_simple`

``` bash
cd /Users/ian/GitHub/refs/iceoryx-rs/iceoryx/

# 中间主进程
# target/iceoryx-install/bin/iox-roudi

# 消息发布者
# target/debug/examples/publisher_simple

# 消息订阅者
# target/debug/examples/subscriber_simple

```
