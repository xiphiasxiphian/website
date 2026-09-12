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

  const width = 1024;
  const height = 768;

  const width_string = `${width}px`;
  const height_string = `${height}px`;

  return (
    <div style={{ display: 'flex', flexDirection: 'column', width: width_string, height: height_string, margin: 0 }}>
      <header style={{ padding: '20px', background: '#333', color: 'white' }}>
        <h1>Test</h1>
      </header>

      {/* The WebGL Canvas */}
      <main style={{ flexGrow: 1 }}>
        <canvas
          id="map-canvas"
          width={width}
          height={height}
          style={{ width: width_string, height: height_string, display: 'block' }}
        ></canvas>
      </main>
    </div>
  );
});
