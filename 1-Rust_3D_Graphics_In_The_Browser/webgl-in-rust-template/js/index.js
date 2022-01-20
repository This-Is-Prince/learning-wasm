const rust = import("../pkg/index.js");
const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext("webgl", { antialias: true });

rust
  .then((m) => {
    if (!gl) {
      alert("Failed to initialize WebGL");
      return;
    }

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
    const princeClient = new m.PrinceClient();
    const initialTime = Date.now();
    let lastDrawTime = -1; // In milliseconds

    function render() {
      window.requestAnimationFrame(render);
      const currTime = Date.now();

      if (currTime >= lastDrawTime + FPS_THROTTLE) {
        lastDrawTime = currTime;
        const { innerHeight, innerWidth } = window;

        if (
          window.innerHeight != canvas.height ||
          window.innerWidth != canvas.width
        ) {
          canvas.height = innerHeight;
          canvas.clientHeight = innerHeight;
          canvas.style.height = innerHeight;

          canvas.width = innerWidth;
          canvas.clientWidth = innerWidth;
          canvas.style.width = innerWidth;

          gl.viewport(0, 0, innerWidth, innerHeight);
        }
        let elapsedTime = currTime - initialTime;
        princeClient.update(elapsedTime, innerHeight, innerWidth);
        princeClient.render();
        // Rust Update call
        // Rust Render call
      }
    }
    render();
  })
  .catch(console.error);
