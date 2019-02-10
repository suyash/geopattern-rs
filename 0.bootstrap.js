(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./pkg */ \"./pkg/geopattern.js\");\n\n\nconst container = document.querySelector(\"#container\");\nconst input = document.querySelector(\"#input\");\n\nconst image = _pkg__WEBPACK_IMPORTED_MODULE_0__[\"generate_base64_svg_string\"](\"\");\ncontainer.style.background = `url(\"data:image/svg+xml;base64,${image}\")`;\n\ninput.addEventListener(\"keyup\", () => {\n    const image = _pkg__WEBPACK_IMPORTED_MODULE_0__[\"generate_base64_svg_string\"](input.value);\n    container.style.background = `url(\"data:image/svg+xml;base64,${image}\")`;\n});\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./pkg/geopattern.js":
/*!***************************!*\
  !*** ./pkg/geopattern.js ***!
  \***************************/
/*! exports provided: generate_minified_svg_string, generate_base64_svg_string */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"generate_minified_svg_string\", function() { return generate_minified_svg_string; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"generate_base64_svg_string\", function() { return generate_base64_svg_string; });\n/* harmony import */ var _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./geopattern_bg */ \"./pkg/geopattern_bg.wasm\");\n/* tslint:disable */\n\n\nlet cachedTextEncoder = new TextEncoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passStringToWasm(arg) {\n\n    const buf = cachedTextEncoder.encode(arg);\n    const ptr = _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](buf.length);\n    getUint8Memory().set(buf, ptr);\n    WASM_VECTOR_LEN = buf.length;\n    return ptr;\n}\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nlet cachedGlobalArgumentPtr = null;\nfunction globalArgumentPtr() {\n    if (cachedGlobalArgumentPtr === null) {\n        cachedGlobalArgumentPtr = _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_global_argument_ptr\"]();\n    }\n    return cachedGlobalArgumentPtr;\n}\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n/**\n* @param {string} arg0\n* @returns {string}\n*/\nfunction generate_minified_svg_string(arg0) {\n    const ptr0 = passStringToWasm(arg0);\n    const len0 = WASM_VECTOR_LEN;\n    const retptr = globalArgumentPtr();\n    try {\n        _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"generate_minified_svg_string\"](retptr, ptr0, len0);\n        const mem = getUint32Memory();\n        const rustptr = mem[retptr / 4];\n        const rustlen = mem[retptr / 4 + 1];\n\n        const realRet = getStringFromWasm(rustptr, rustlen).slice();\n        _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n        return realRet;\n\n\n    } finally {\n        _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr0, len0 * 1);\n\n    }\n\n}\n\n/**\n* @param {string} arg0\n* @returns {string}\n*/\nfunction generate_base64_svg_string(arg0) {\n    const ptr0 = passStringToWasm(arg0);\n    const len0 = WASM_VECTOR_LEN;\n    const retptr = globalArgumentPtr();\n    try {\n        _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"generate_base64_svg_string\"](retptr, ptr0, len0);\n        const mem = getUint32Memory();\n        const rustptr = mem[retptr / 4];\n        const rustlen = mem[retptr / 4 + 1];\n\n        const realRet = getStringFromWasm(rustptr, rustlen).slice();\n        _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n        return realRet;\n\n\n    } finally {\n        _geopattern_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](ptr0, len0 * 1);\n\n    }\n\n}\n\n\n\n//# sourceURL=webpack:///./pkg/geopattern.js?");

/***/ }),

/***/ "./pkg/geopattern_bg.wasm":
/*!********************************!*\
  !*** ./pkg/geopattern_bg.wasm ***!
  \********************************/
/*! exports provided: memory, __wbindgen_global_argument_ptr, generate_minified_svg_string, generate_base64_svg_string, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./pkg/geopattern_bg.wasm?");

/***/ })

}]);