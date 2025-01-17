<!--
 * @Author       : sunzhifeng <ian.sun@auodigitech.com>
 * @Date         : 2021-08-25 21:35:39
 * @LastEditors  : ian <lauer3912@gmail.com>
 * @LastEditTime : 2021-12-18 23:32:00
 * @FilePath     : /iceoryx-rs/README.zh-CN.md
 * @Description  : Created by sunzhifeng, Please coding something here
-->

## 准备条件，编译 iceoryx

### gitpod

参见 ./scripts/ 目录中的脚本

```bash

# 升级rust
source ./scripts/upgrade_rust.sh

# 安装swift编译环境
source ./scripts/install_swift.sh

```

### Linux

#### 编译准备条件

```bash
sudo apt install gcc g++ cmake libacl1-dev libncurses5-dev pkg-config
```

#### 编译命令

关于 iceoryx 的配置说明

<https://github.com/eclipse-iceoryx/iceoryx/blob/master/doc/website/advanced/configuration-guide.md>

```bash
cd iceoryx/
cmake -Bbuild -Hiceoryx_meta
cmake -Bbuild -Hiceoryx_meta -DCMAKE_PREFIX_PATH=$(PWD)/build/dependencies/ -DIOX_MAX_CHUNKS_HELD_PER_SUBSCRIBER_SIMULTANEOUSLY=4
cmake --build build -j 4

# 默认安装到`/usr/local`下
# 例如: 
# /usr/local/bin/iox-roudi
# /usr/local/lib/libiceoryx_posh.a
# /usr/local/include//iceoryx_binding_c
# /usr/local/share/doc/iceoryx_binding_c/LICENSE
sudo cmake --build build -j 4 --target install 

# cmkae 可以指定安装的目录
# 设置 DESTINATION 环境变量
# TODO: 注释一下

```

### macOS

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
  cd iceoryx
  ICEORYX_DIR=$PWD
  mkdir -p build
  cd build
  git clone https://github.com/mirror/ncurses.git
  cd ncurses
  git checkout v6.2
  ./configure  --prefix=$ICEORYX_DIR/build/dependencies/ --exec-prefix=$ICEORYX_DIR/build/dependencies/ --with-termlib
  make -j12
  make install
  cd ../../
  # /Users/ian/GitHub/refs/iceoryx-rs/iceoryx
  cmake -Bbuild -Hiceoryx_meta -DCMAKE_PREFIX_PATH=$(PWD)/build/dependencies/
  cmake --build build -j 4
  cd ../
  # /Users/ian/GitHub/refs/iceoryx-rs
  cargo build --all --examples
```

## 1. Mac 构建说明

### 1.1. 准备条件

#### 1.1.1. 下载源码

- 源码下载地址：<https://github.com/elBoberido/iceoryx-rs.git>

``` bash
  cd /Users/ian/GitHub/refs/
  git clone https://github.com/elBoberido/iceoryx-rs.git --recursive
  ls -l
```

#### 1.1.2. 构建基础的 iceoryx 源码，并且安装依赖

```bash
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
  # 构建编译 iceoryx，生成必要的 include, bin, lib 等目录
  cmake -Bbuild -Hiceoryx_meta -DCMAKE_PREFIX_PATH=$(PWD)/build/dependencies/
  cmake --build build -j 4
```

#### 1.1.3. 编译 examples 目录

```bash
  cd "/Users/ian/GitHub/refs/iceoryx-rs/iceoryx/"
  ICEORYX_DIR=$PWD
  # 使用工具构建 iceoryx 项目的 examples
  ./tools/iceoryx_build_test.sh examples

```

#### 1.1.4. 构建动态库

```bash
  cd "/Users/ian/GitHub/refs/iceoryx-rs/iceoryx/"
  ICEORYX_DIR=$PWD
  # 使用工具构建 iceoryx 项目的
  ./tools/iceoryx_build_test.sh build-shared

```


### 编译自己的试验项目

参见：[./iceoryx/iceoryx_examples/icedelivery_iantry_in_c/README.md](./iceoryx/iceoryx_examples/icedelivery_iantry_in_c/README.md)

用来测试其他进程，通过其他语言实现的，如Rust实现的Iceoryx进程，与C语言实现的Iceoryx进程的数据交流能否正常？

``` bash
# 定位目录
cd icedelivery_iantry_in_c

# 运行cmake, 生成的文件都存储到build目录，如:Makefile
cmake . -Bbuild

# 编译, 生成目录为build
cmake --build build --target all

# 验证运行情况

#1. iox-roudi 先运行
#2. 运行 icedelivery_iantry_in_c/build/iox-c-publisher
#3. 运行 icedelivery_iantry_in_c/build/iox-c-subscriber

# 定位到iceoryx-rs/target/debug/examples目录，运行 subscriber_simple

```

### 1.2. 编译构建 iceoryx-rs 项目

```bash
  ####### 构建 iceoryx-rs 项目 #######
  cd ../
  # 所在路径：/Users/ian/GitHub/refs/iceoryx-rs
  # 构建 debug 版本
  cargo build --all --examples

  # 构建 release 版本
  cargo build --all --release --examples

  # 清除临时生成文件，并编译
  rm -fr ./target/debug && cargo build --all --examples
```

## 2. 运行 Demo

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

## 3. Rust 编译的进程与 C 编译的进程互动

参见：iceoryx/iceoryx_examples/icedelivery_iantry_in_c/

- 启动 iox-roudi

    ``` bash
    cd /Users/ian/GitHub/refs/iceoryx-rs/iceoryx/
    ./target/iceoryx-install/bin/iox-roudi
    ```

- 启动 rust 编写的消息发布者

    ``` bash
    cd /Users/ian/GitHub/refs/iceoryx-rs/iceoryx/
    ./target/debug/examples/publisher_simple
    ```

- 启动 c 语言编写的消息订阅者

    ``` bash
    cd /Users/ian/GitHub/refs/iceoryx-rs/iceoryx/iceoryx_examples/icedelivery_iantry_in_c/
    ./build/iox-c-subscriber
    ```

## Demo 测试

### 编译

```bash
cargo build --all --examples
```

## 运行

### 常用操作

- start RouDi

```bash
./target/iceoryx-install/bin/iox-roudi -c ./roudi_config.toml 
```

- start

```bash
./elBoberido_iceray/target/debug/iceray
```

- start the publisher `./target/debug/examples/publisher_simple`
- start a subscriber `./target/debug/examples/subscriber_simple`

### 指定运行配置

