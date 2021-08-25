#

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
  cmake -Bbuild -Hiceoryx_meta -DCMAKE_PREFIX_PATH=$(PWD)/build/dependencies/
  cmake --build build -j 4
  cd ../
  cargo build --all --examples
```