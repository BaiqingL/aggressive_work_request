# Aggressively request work units from F@H servers
This program will wait a certain point before checking for a paused or hanging work unit download. It will then fully reload the client. (Only runs on Linux)
### Usage
./aggressive_work_request (path/to/FAHClient) (path/to/log.txt) (minutes_before_checking)

# 简单的检查跑包状态
等待指定的分钟,如果客户端没有跑包将会重启 (支持Linux)
### 使用方法
./aggressive_work_request (FAHClient 控制脚本位置) (日志文件位置) (检查间隔分钟)
