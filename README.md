# rust_learning
*A great rust learning repository belonging to Open Source AI.*  
## 安装  
需要安装**curl**。
### Linux或者macOS  
两种方法：  

* 参考《RUST权威指南》  
```
$ curl https://sh.rustup.rs -sSf | sh
```
此方法亲测速度较慢，可以挂代理解决（可能）。  

* 使用中科大的代理  
Rust Toolchain 反向代理：<https://mirrors.ustc.edu.cn/help/rust-static.html>  
Rust Crates 源使用帮助：<https://mirrors.ustc.edu.cn/help/crates.io-index.html>  
Rust 官网：<https://www.rust-lang.org/learn/get-started>  
```
$ echo "export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static" >> ~/.bashrc  
$ echo "export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup" >> ~/.bashrc  
$ source ~/.bashrc  
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
```  
此方法亲测速度飞快，推荐。

安装过程会自动将Rust工具链接加到环境变量PATH中，重启终端后配置生效。  

### Windows  
参考Rust官网<https://www.rust-lang.org/>。  

##  更新和卸载  
### Linux或者macOS
```
$ rustup update
$ rustup self uninstall  
```  

### Windows  
正常卸载。  










