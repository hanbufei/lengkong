## 一 开发流程
### （一）初始化数据库
1. 新建数据库(默认连接密码为：root/lengkong#CMS1234)，运行命令如下：
> CREATE DATABASE IF NOT EXISTS `cms` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
2. 新建表，运行backup/cms.sql。
### （二）开发管理端项目gui
1. 初始化项目

Gui程序采用tauri-solid-ts-tailwind-vite脚手架。从github下载后重命名为gui，然后参考readme进行相关工具和库的更新。

下载地址：https://github.com/AR10Dev/tauri-solid-ts-tailwind-vite 。

2. 布局

![alt text](img/2.png)

在src\App.tsx中通过tailwindcss完成整体布局。

3. 开发进度
- 2月17日：完成全局消息。完成生命周期。完成js里调用rust代码的测试。
- 2月18日：添加登录、管理、新增三个页面并配置路由与导航。完成菜单栏。完成登录页面。 
  
  ![alt text](img/3.png)
  
- 2月19日:
  + 新建dao目录用于数据库交互。
  + 新建service目录(套tauri的command宏)用于逻辑处理和js调用。
  + 完成gui页面调用数据库查询的测试。

- 2月21日:完成数据库中全部表的dao层和service层的增删改查等基本操作。