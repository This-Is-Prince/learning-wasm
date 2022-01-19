import("../pkg/index.js")
  .then((module) => {
    module.start();
  })
  .catch(console.error);
