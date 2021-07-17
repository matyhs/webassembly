let importObject = {};
WebAssembly.instantiateStreaming(fetch('./calc.wasm'), importObject)
.then(obj => {
  console.log(obj.instance.exports.add(2,3));
  console.log(obj.instance.exports.mul(2,3));
  console.log(obj.instance.exports.div(10,2));
  console.log(obj.instance.exports.sub(10,2));
});