# 更新Rust到最新版本，并修改整体cargo使用的版本
WORKDIR=$PWD
echo "WORKDIR=$WORKDIR"

cd "/workspace/.cargo/bin"
./rustup update stable  #
./rustup default stable

# 打印环境版本
cargo -V
rustc -V

# 切换会原先的工作目录
cd "$WORKDIR"