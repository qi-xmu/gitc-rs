# Git Commit Helper 注释提交助手

使用说明：Git Commit Helper是一个用于生成Git提交注释的工具，旨在帮助开发者遵循一致的提交注释格式。

默认遵循的提交注释格式如下：
```
feat(<具体范围>): <提炼的关键描述>
fix(<具体范围>): <提炼的关键描述>
docs(<具体范围>): <提炼的关键描述>
style(<具体范围>): <提炼的关键描述>
refactor(<具体范围>): <提炼的关键描述>
test(<具体范围>): <提炼的关键描述>
chore(<具体范围>): <提炼的关键描述>
```

# 使用方法

1. 在 Coze.cn 复制机器人 [GitCommitter](https://www.coze.cn/store/bot/7376177473538736182?panel=1&bid=6cne5n954701i)
2. 获取机器人的 `bot_id` 和 `token`.
3. 填写 `～/.gitc` 文件中的 `bot_id` 和 `token`.
4. 使用 `gitc` 命令替换 `git commit` 提交代码.