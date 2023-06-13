# auto-youth-learning

> 重庆青年大学习自动学习

开启后会在每天零点自动打卡。需要 openid，请自行抓包获取。

## docker

> docker run -itd --name auto-study -e OPENID=\<你的openid> arcticrain/auto-youth-learning

## 手动部署

请自行配置环境变量 `OPENID`