# MHGU Palico No Aggro / 随从猫无仇恨 v0.1.0

[下载安装包](https://github.com/Akumi-xu/MHGU-Palico-No-Aggro/releases/download/v0.1.0/MHGU_Palico_No_Aggro_v0.1.0-Recomp.zip) · [版本与历史发布](https://github.com/Akumi-xu/MHGU-Palico-No-Aggro/releases)

Clear AI Palico hate and exclude companions from monster target selection in offline solo hunts. Controlled Prowlers remain valid player targets. Incidental / area damage still applies.

离线单人时清除AI随从猫仇恨，并在怪物选定/提交目标时排除AI猫。兼容原版零至两只随从和单人三随从猫。主控猎猫照常被攻击；猫的攻击、支援及力尽恢复仍由原版处理。

## 下载与安装

在 [Releases](https://github.com/Akumi-xu/MHGU-Palico-No-Aggro/releases) 下载安装ZIP。将包内romfs放到游戏目录 mods/PALICO_NO_AGGRO/ 下，在MOD管理中启用并重启。不要同时安装两份同名NRO。无需Switch加载器，不替换exe/dll。无INI配置。

已验证 PC MHGURecomp 0.16.0 / MHGU 1.4.0。仅离线单人生效。安卓、模拟器、Switch和实际联机未验证。卸载时退出游戏并移除本MOD目录，再重启。

## 构建

源码目录必须为纯ASCII路径。需要 Rust nightly + rust-src、ARM GCC和cargo-forge；Rust依赖 mhgu-forge =1.5.0，其余版本由Cargo.lock固定。开发验证工具链：rustc 1.101.0-nightly (12433c307 2026-10-24)、ARM GCC 15.2.1。

将ARM GCC的bin加入PATH，在源码根目录执行：

```powershell
cargo forge build --release
```

产物为根目录palico_no_aggro.nro。请保留.cargo、link、目标JSON、build.rs、Cargo.toml/Cargo.lock和rust-toolchain.toml。源码ZIP和仓库均包含全部构建必需文件。

主机规则测试（显式指定主机目标，避免项目.cargo的ARM默认目标）：

```powershell
rustc --test --target x86_64-pc-windows-gnu src/rules.rs -o rules_tests.exe
./rules_tests.exe
```

重定位检查：

```powershell
python tools/check_relocations.py palico_no_aggro.nro
```

## 实现与兼容

四个Forge插件偏移：BB7A0阻止猫仇恨增加；BBF98清除四个猫仇恨槽；C5B14过滤选定目标；C6E14处理直接/脚本提交。重新调用原生猎人选择器处理有效性及同区判断；没有有效猎人时由原生路径保留空目标。

AI随从为原生目标类型2，主控猎猫属于玩家类型1。B70A4还用于猫自身战斗与支援感知，保持原样。启动时校验入口指令，不匹配则停用。与修改相同四个入口的其他插件可能冲突。

## 验证范围

本版为Pre-release测试版，详见 [VALIDATION.md](VALIDATION.md)。构建、规则测试和NRO重定位PASS。零/一/两只随从、猎人三随从及主控猎猫场景共780次自然状态采样，未发现猫仇恨或猫锁定；另有受控回复、挑衅优先及完整力尽恢复检查。采样不是连续逐帧追踪，也不是780次独立选目标事件。

未穷尽全部怪种、特殊脚本/任务、装备及支援组合；范围、固定轨迹和惯性攻击仍可波及猫。没有完整实测“猎人不在同区而猫长期单独留场”。

## 致谢

使用 Fexty12573 的 [Forge](https://github.com/Fexty12573/forge) / mhgu-forge，Codex协助实现与测试。安装包仅包含自行构建插件与说明；源码不含游戏文件、生成C、存档或密钥。

仓库保存展开的源码与构建材料；安装包统一从 Release 下载。历史版本及标签保持原样。
