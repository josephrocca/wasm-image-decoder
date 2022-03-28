import init, {initThreadPool, decode} from "./wasm/wasm_image_decoder.js";
await init();
await initThreadPool(4);

export default function(buffer) {
  if(buffer.constructor !== Uint8Array) {
    throw new Error("Expecting an Uint8Array as input");
  }
  let result = decode(buffer);
  let data = result.slice(0, -4);
  let widthBytes = result.slice(-4);
  let width = (widthBytes[0] << 24) + (widthBytes[1] << 16) + (widthBytes[2] << 8) + (widthBytes[3] << 0);
  let imageData = {
    width,
    height: (data.length/4) / width,
    data,
  };
  return imageData;
};
