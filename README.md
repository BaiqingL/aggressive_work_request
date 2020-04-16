# 简单的检查跑包状态
等待指定的分钟,如果客户端没有跑包将会重启 (支持Linux)
### 使用方法
./aggressive_work_request （设置ini文件位置）

### 使用设置
* slot  那些工作卡会被监控
* SleepInterval 检查工作状态间隔分钟
* FAHLocation   F@H控制脚本位置
* LogLocation   日志位置
* IdleTimeout   待机间隔最久时间
* DownloadTimeout   下载进度停止最久时间（防止卡包）
* UploadTimeout 上传进度停止最久间隔时间
* ReloadThreshold   工作卡（FSXX）违规间隔时间上限
假如你有4张显卡，可以将ReloadThreshold设置为3，这样只有三张显卡待机，下载卡包或者上传失误才会重启客户端。


# Aggressively request work units from F@H servers
This program will wait a certain point before checking for a paused or hanging work unit download. It will then fully reload the client. (Only runs on Linux)
### Usage
./aggressive_work_request path/to/conf.ini

### Configuration
There is a conf.ini file provided with configuration details within the file
Modify the options according to specific needs
* Define sltos within the configuration file
* FAHLocation is the path to FAHClient shell control script
* LogLocation is the path to the FAHClient log file
* Idle, Download, Upload thresholds are how long to allow each slot to spend on specific tasks
* SleepInterval means how long to pause between each check to see if there is any violation of previously defined intervals
* ReloadThreshold means how many slots need to violate definied rules before the client gets actually reloaded
