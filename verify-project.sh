#!/bin/bash

echo "========================================="
echo "Tauri 登录捕获项目验证脚本"
echo "========================================="
echo ""

# 检查必要文件
echo "检查项目文件..."
files=(
    "package.json"
    "vite.config.js"
    "index.html"
    "src/main.js"
    "src/App.vue"
    "src-tauri/Cargo.toml"
    "src-tauri/tauri.conf.json"
    "src-tauri/src/main.rs"
    "src-tauri/build.rs"
    "README.md"
    "SETUP.md"
    ".gitignore"
)

all_files_exist=true
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "✓ $file"
    else
        echo "✗ $file - 缺失"
        all_files_exist=false
    fi
done

echo ""
echo "检查图标文件..."
icon_files=(
    "src-tauri/icons/32x32.png"
    "src-tauri/icons/128x128.png"
    "src-tauri/icons/128x128@2x.png"
    "src-tauri/icons/icon.ico"
    "src-tauri/icons/icon.icns"
)

all_icons_exist=true
for icon in "${icon_files[@]}"; do
    if [ -f "$icon" ]; then
        echo "✓ $icon"
    else
        echo "✗ $icon - 缺失"
        all_icons_exist=false
    fi
done

echo ""
echo "检查依赖..."
if [ -d "node_modules" ]; then
    echo "✓ Node.js 依赖已安装"
else
    echo "✗ Node.js 依赖未安装 - 运行 'pnpm install'"
fi

if [ -f "pnpm-lock.yaml" ]; then
    echo "✓ pnpm-lock.yaml 存在"
fi

echo ""
echo "检查 Rust 编译..."
cd src-tauri
if cargo check --quiet 2>/dev/null; then
    echo "✓ Rust 代码编译检查通过"
else
    echo "⚠ Rust 代码编译检查失败（可能需要先构建）"
fi
cd ..

echo ""
echo "========================================="
if $all_files_exist && $all_icons_exist; then
    echo "✓ 项目验证通过！"
    echo ""
    echo "运行项目:"
    echo "  pnpm tauri dev"
    echo ""
    echo "或查看 SETUP.md 了解更多信息"
else
    echo "✗ 项目验证失败，请检查缺失的文件"
fi
echo "========================================="
