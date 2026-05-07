#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');

// プラットフォームとアーキテクチャを取得
const platform = os.platform(); // 'win32', 'darwin', 'linux'
const arch = os.arch();         // 'x64', 'arm64' etc.

// 実行するバイナリの名前を決定
let binaryName = 'oshirase-mcp-linux-x64'; // デフォルトフォールバック

if (platform === 'win32') {
    binaryName = 'oshirase-mcp-win-x64.exe';
} else if (platform === 'darwin') {
    binaryName = arch === 'arm64' ? 'oshirase-mcp-macos-arm64' : 'oshirase-mcp-macos-x64';
} else if (platform === 'linux') {
    binaryName = arch === 'arm64' ? 'oshirase-mcp-linux-arm64' : 'oshirase-mcp-linux-x64';
}

const binaryPath = path.join(__dirname, binaryName);

// 元の引数をそのままバイナリに渡す
const args = process.argv.slice(2);

// サブプロセスとしてバイナリを起動
const child = spawn(binaryPath, args, {
    stdio: 'inherit',
    windowsHide: true
});

child.on('error', (err) => {
    console.error(`[ERROR] Failed to start MCP server binary at ${binaryPath}:`, err.message);
    process.exit(1);
});

child.on('exit', (code) => {
    process.exit(code || 0);
});
