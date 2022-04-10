const nodeBinary = require("./rust-package/index.node");

setInterval(() => {
    console.log(nodeBinary.GetCursorPos(), nodeBinary.GetAsyncKeyState(0x01), nodeBinary.GetAsyncKeyState(0x04), nodeBinary.GetAsyncKeyState(0x02));
}, 100);
