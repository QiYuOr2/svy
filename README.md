# svy 搞个自己用的工具合集

## Usage

### 本地 link

```
pnpm build

pnpm link --global

svy
```

### 开发

```
pnpm svy
```

根据模板创建命令

在项目根目录下执行

```
pnpm svy create <命令名称>
```

会在`src/modules`下创建一个命令文件

通过这种方式创建的命令会被自动引入
