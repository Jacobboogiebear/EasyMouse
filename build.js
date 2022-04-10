const child_process = require('child_process');
const path = require('path');
const fs = require('fs');

child_process.spawnSync("npm.cmd", ["run", "build"], {
    cwd: path.join(__dirname, './rust-package')
})
fs.copyFileSync(path.join(__dirname, './rust-package/index.node'), path.join(__dirname, './index.node'));