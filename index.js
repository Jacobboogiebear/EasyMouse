const nodeBinary = require('./index.node');

module.exports = {
    GetCursorPos: nodeBinary.GetCursorPos,
    GetAsyncKeyState: nodeBinary.GetAsyncKeyState,
    GetRightMouseButton: nodeBinary.GetRightMouseButton,
    GetLeftMouseButton: nodeBinary.GetLeftMouseButton,
    GetMiddleMouseButton: nodeBinary.GetMiddleMouseButton,
}