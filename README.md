# S5 Web Proxy

The S5 Web Proxy can securely stream files of any size from the S5 network, directly in your web browser. It should work in all major web browsers, including Chrome, Brave, Firefox and Safari.

It works by registering and running a service worker which intercepts specific requests, looks up download urls for them and then verifies their integrity with Rust BLAKE3/bao running in WASM.

After adding the service worker to your web app, streaming files from the S5 network is as easy as using the URL `/s5/blob/YOUR_CID_HERE??mediaType=image%2Fpng` in `img` tags, for your video player or just fetching it directly! Encrypted files are supported too.

## Example apps using this proxy

### https://tube5.app

Decentralized video platform, uses the s5 web proxy for streaming video, audio, images and text.

### https://s5.cx

For very basic file sharing.

Encrypted file example: https://s5.cx/#urqYSH3y5m46EwJXosYFqTKbdUxdNvDiQmBVkIv66zeC-fvoqfiFfLts4nrbb76hHcMHVFrdjHENxXl-hSRpo2OLiyGOACQEAJh-OD74rQfX3DhpXiYHEoFDgC8z87vmHfxLdaIEXRx8DJYD2Qgg?mediaType=video%2Fmp4

### https://map-demo.sfive.cloud/

Web-based map view, powered by the PMTiles format and the S5 web proxy.

## Use the proxy in your application

1. Copy the `sw.js` and `rust_lib.wasm` files in the `static/` directory to the root of your web app.

2. Register the service worker in your JS/TS code, for example like this:
```js
const sw = navigator.serviceWorker;
if (!sw) {
  alert('Service Worker API not supported')
} else {
  if (!sw.controller) {
    try {
      sw.register('sw.js').then(function (registration) {
        console.log('Registration succeeded.', registration);
        // ! Optional, makes sense for static web apps
        // window.location.reload();
      })
    } catch (e) {
      alert(e)
    }
  }
}
```

3. Stream files from the S5 network using normal URLs and web apis, for example:
```html
<video controls>
  <source src="/s5/blob/uJh9dvBupLgWG3p8CGJ1VR8PLnZvJQedolo8ktb027PrlTT5LvAY?mediaType=video%2Fmp4" type="video/mp4">  
</video>

