# 简单的检查跑包状态
等待指定的分钟,如果客户端没有跑包将会重启 (支持Linux)
### 使用方法
./aggressive_work_request (FAHClient 控制脚本位置) (日志文件位置) (检查间隔分钟)


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
