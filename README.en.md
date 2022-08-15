# Open-WSL-Shell-Command-Extensions
[![](https://img.shields.io/badge/English-%E4%B8%AD%E6%96%87-green.svg)](README.md)[![](https://img.shields.io/badge/License-MulanPSL%202.0-green.svg)](LICENSE_en)
#### Description
The Open-WSL-Shell-Command-Extensions is developed based on advanced features such as file intercommunication and command interuse of the Windows Subsystem for Linux (WSL), which aims to make the development experience smoother under the WSL. 

For example, if you want to switch your workspace to another folder in the Linux terminal of WSL, this can usually be done with the `cd` command in Linux. However, working under Windows, the path you want to switch to is in a Windows path that needs to be copied and pasted from an application running on Windows, such as `Explorer` or `VSCode`. 
At this time, the project provides the 'cdwin' tool that can solve the problem. After installing this project, you can easily switch between `D:\workspace\my_project` and `C:\Windows` by simply replacing `cd` with `cdwin` in WSL terminals such as "bash" or "zsh". Instead of manually typing `cd /mnt/d/workspace/my_project` or `/mnt/c/Windows`, which would have to do the tedious work of replacing `\` with`/`, adding `/ MNT/`, and having to be strictly case-sensitive.

#### Software Architecture
Software architecture description
We used Rust Programming Language with std for this project. 

#### Installation

1.  xxxx
2.  xxxx
3.  xxxx

#### Instructions

1.  xxxx
2.  xxxx
3.  xxxx

#### Contribution

1.  Fork the repository
2.  Create Feat_xxx branch
3.  Commit your code
4.  Create Pull Request


#### Ingenious Features

