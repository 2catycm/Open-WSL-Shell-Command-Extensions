# 适用于-适用于Linux的Windows子系统-的开放扩展终端命令集
[![](https://img.shields.io/badge/%E4%B8%AD%E6%96%87-English-green.svg)](README_en.md)[![](https://img.shields.io/badge/License-MulanPSL%202.0-green.svg)](LICENSE)
#### 介绍
基于WSL(Windows Subsystem for Linux，适用于Linux的Windows子系统 )文件互通、指令互用等高级特性开发的开放扩展终端命令集，旨在让 WSL 下的开发体验更加顺畅。

举个例子，您想在wsl的Linux终端中切换您的工作区到另一个文件夹，通常这可以通过Linux下的`cd`命令完成。然而，在Windows下工作的您希望切换的路径在某个Windows路径，而该路径需要从Windows上运行的应用复制粘贴得到，比如`资源管理器`或`VSCode`中复制得到。这个时候，本项目提供的`cdwin`工具就可以解决这一问题。安装了本项目之后，您只需要把`cd`换成`cdwin`, 在`bash`或者`zsh`等wsl终端中便能轻松切换"D:\workspace\my_project" "C:\Windows"这样的文件夹，而不是手动输入`cd /mnt/d/workspace/my_project` 或 `/mnt/c/Windows`, which 原本需要做 把`\`换成`/`, 加上`/mnt/`, 以及必须严格区分大小写等繁琐工作。
#### 软件架构
软件架构说明

使用 Rust 语言的标准特性进行编程。

#### 安装教程

1.  xxxx
2.  xxxx
3.  xxxx

#### 使用说明

1.  xxxx
2.  xxxx
3.  xxxx

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

