const nodeBinary = require("./rust-package/index.node");

setInterval(() => {
    console.log(nodeBinary.GetCursorPos(), nodeBinary.GetLeftMouseButton(), nodeBinary.GetMiddleMouseButton(), nodeBinary.GetRightMouseButton());
}, 100);
