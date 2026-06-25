import { component$, useVisibleTask$ } from '@builder.io/qwik';
import init, { start_map_engine } from '../wasm/map/map.js';

export default component$(() => {

  useVisibleTask$(async () => {
    try {
      await init();

      // canvas handover
      start_map_engine('map-canvas');

      console.log("WASM WebGL Engine Running!");
    } catch (error) {
      console.error("Failed to start map engine:", error);
    }
  });

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh', margin: 0 }}>
      <header style={{ padding: '20px', background: '#333', color: 'white' }}>
        <h1>Test</h1>
      </header>

      {/* The WebGL Canvas powered by Rust */}
      <main style={{ flexGrow: 1 }}>
        <canvas
          id="map-canvas"
          width={1024}
          height={768}
          style={{ width: '100%', height: '100%', display: 'block' }}
        ></canvas>
      </main>
    </div>
  );
});
