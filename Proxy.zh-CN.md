<!--
 * @Author       : ian <lauer3912@gmail.com>
 * @Date         : 2021-12-17 20:56:55
 * @LastEditors  : ian <lauer3912@gmail.com>
 * @LastEditTime : 2021-12-18 14:15:39
 * @FilePath     : /iceoryx-rs/Proxy.zh-CN.md
 * @Description  : 添加文件描述内容
-->

# 通过Privoxy把socks5代理转换成http代理

官方网站：https://www.privoxy.org/

通过Privoxy把socks5代理转换成http代理

最近玩上了trojan，据说安全性超过了SS和SSR，至少目前电信没给我打电话通知我家里网络状况异常了。trojan比较新，客户端比较少，在Mac上能用，但是只能提供sock5代理。在一些软件里比如iTerm、Android Studio里都有对https代理的需求。今天我学着别人通过Privoxy将socks5代理转换成http代理。
安装Privoxy

Privoxy是一个 HTTP 协议过滤代理，常结合 Tor 使用。Privoxy 是有着先进的过滤能力和保护隐私的代理工具，它可以过滤网页内容，管理cookies，控制访问，除广告、横幅、弹出窗口等等，它同时支持单系统和多用户网络。
当用户直接使用 SOCKS 代理访问网络时，浏览器会泄漏 DNS 请求，降低匿名性，这时应该使用 Privoxy。

    Privoxy is a non-caching web proxy with advanced filtering capabilities for enhancing privacy, modifying web page data and HTTP headers, controlling access, and removing ads and other obnoxious Internet junk.

通过brew来安装Privoxy

-> brew install privoxy
Updating Homebrew...
==> Downloading https://mirrors.ustc.edu.cn/homebrew-bottles/bottles/privoxy-3.0
Already downloaded: /Users/ted/Library/Caches/Homebrew/downloads/6d09ad8bd36fc3b523c9067d81fffe0f767258dc0e85295b97d0e0110df9dbce--privoxy-3.0.28.catalina.bottle.tar.gz
==> Pouring privoxy-3.0.28.catalina.bottle.tar.gz
==> Caveats
To have launchd start privoxy now and restart at login:
  brew services start privoxy
Or, if you don't want/need a background service you can just run:
  privoxy /usr/local/etc/privoxy/config
==> Summary
🍺  /usr/local/Cellar/privoxy/3.0.28: 98 files, 2MB

配置Privoxy

按照安装时提示的配置文件路径 /usr/local/etc/privoxy/config ，在尾部增加配置如下:

forward-socks5 / localhost:1180 . #设置转发到本地的socks5代理客户端端口
listen-address 0.0.0.0:1081 #privoxy暴露的HTTP代理地址，设置 privoxy 监听任意 ip的1081端口
forward 10.*.*.*/ . #内网地址不走代理
forward .abc.com/ . #指定域名不走代理

启动Privoxy

按照安装时提示文字，可以通过 brew services start privoxy 来后台运行Privoxy并且配置开机启动，也可以通过 sudo /usr/local/sbin/privoxy /usr/local/etc/privoxy/config 来临时运行Privoxy。
检测是否运行

可以通过 ps aux | grep privoxy 和 lsof -i:1081 来检测Privoxy是否在运行中，其中1081是上面config中配置的监听端口。
查看是否监听端口

netstat -na | grep 1081

如下图表示端口正在监听:
202002192110-1582117845807
退出Privoxy

通过 ps aux | grep privoxy 获取到对应的进程ID，通过 kill 进程ID 的方式即可退出Privoxy。
配置Git走代理
全局配置

git config --global http.proxy http://127.0.0.1:1081
git config --global https.proxy http://127.0.0.1:1081

全局配置取消

git config --global --unset http.proxy
git config --global --unset https.proxy

针对网址配置

git config --global http.https://github.com.proxy socks5://127.0.0.1:1180
git config --global https.https://github.com.proxy socks5://127.0.0.1:1180

针对网址取消

git config --global --unset http.https://github.com.proxy 
git config --global --unset https.https://github.com.proxy

配置全局走代理

直接在.zshrc或者.bash_profile中加入以下代码，source之后，通过对应的方法即可快速开启或者关闭全局代理。

function openProxy() {
    export no_proxy="localhost,127.0.0.1,localaddress,.localdomain.com"
    export http_proxy="http://127.0.0.1:1081"
    export https_proxy=$http_proxy
    echo -e "已开启代理"
}

function closeProxy() {
    unset http_proxy
    unset https_proxy
    echo -e "已关闭代理"
}

全局代理测试

    打开全局代理之后，在iTerm中输入 curl cip.cc , 如果显示的是代理服务器的相关地址信息，就说明代理成功了。
    在iTerm中输入 curl -i https://google.com , 有结果输出，也说明代理成功了。
    202002192144-1582119897596

参考文章：

    Mac 终端通过 privoxy 科学上网
    给 iTerm 设置代理
