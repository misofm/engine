class MisoQualificationGlobalProbe extends AudioWorkletProcessor {
  constructor() {
    super();
    this.port.postMessage({
      textEncoder: typeof TextEncoder,
      webAssembly: typeof WebAssembly,
      renderQuantumSize: typeof globalThis.renderQuantumSize,
    });
  }

  process() {
    return false;
  }
}

registerProcessor("miso-qualification-global-probe", MisoQualificationGlobalProbe);
