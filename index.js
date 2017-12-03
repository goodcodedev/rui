var instance;
var memory;
const POINTER_WIDTH = 4;

function getI32(ptr) {
    var slice = new Uint8Array(memory.buffer).slice(ptr, ptr + POINTER_WIDTH);
    var view = new DataView(slice.buffer);
    return view.getUint32(0, true);
}

var importObject = {
    env: {
        test_js: function(arg) {
            console.log("test_js", arg);
        },
        draw_html: function(ptr) {
            //var ptr = instance.exports.alloc(3 * POINTER_WIDTH);
            var start = getI32(ptr);
            var size = getI32(ptr + POINTER_WIDTH);
            var memView = new Uint8Array(memory.buffer);
            var strData = new Uint8Array(memView.slice(start, start + size));
            var decoder = new TextDecoder("UTF-8");
            var asUtf8 = decoder.decode(strData);
            console.log("draw", asUtf8);
        }
    },
    imports: {
        env: {
            test_js: function() {
                console.log("test_js");
            }
        }
    }
}

fetch('main.wasm').then(response => response.arrayBuffer())
    .then(bytes => {
        return WebAssembly.instantiate(bytes, importObject)
    })
    .then(results => {
        instance = results.instance;
        memory = instance.exports.memory;
        console.log(instance);
        results.instance.exports.browser()
    });

console.log("Test");