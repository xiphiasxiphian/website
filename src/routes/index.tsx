// import { component$ } from "@builder.io/qwik";
// import type { DocumentHead } from "@builder.io/qwik-city";

// import "./styles.css";
// import { Carousel, CarouselItem } from "~/components/carousel";

// // Data
// const PROJECTS: CarouselItem[] = [
//   { icon: "🚀", title: "Test 1", desc: "Description 1" },
//   { icon: "🎮", title: "Test 2", desc: "Description 2" },
//   { icon: "🤖", title: "Test 3", desc: "Description 3" },
//   { icon: "📊", title: "Test 4", desc: "Description 4" },
//   { icon: "🔐", title: "Test 5", desc: "Description 5" },
// ];

// const SKILLS: CarouselItem[] = [
//   { icon: "⚡", title: "Test 1", desc: "Description 1" },
//   { icon: "🌐", title: "Test 2", desc: "Description 2" },
//   { icon: "🗄️", title: "Test 3", desc: "Description 3" },
//   { icon: "☁️", title: "Test 4", desc: "Description 4" },
//   { icon: "🎨", title: "Test 5", desc: "Description 5" },
//   { icon: "🧠", title: "Test 6", desc: "Description 6" },
// ];

// const LANGUAGES: CarouselItem[] = [
//   { icon: "🟦", title: "Test 1", desc: "Description 1" },
//   { icon: "🐍", title: "Test 2", desc: "Description 2" },
//   { icon: "☕", title: "Test 3", desc: "Description 3" },
//   { icon: "🦀", title: "Test 4", desc: "Description 4" },
//   { icon: "🐹", title: "Test 5", desc: "Description 5" },
// ];

// export default component$(() => {
//   return (
//     <>
//       <Carousel id="projects" title="My Projects" items={PROJECTS} />

//       <Carousel id="skills" title="My Skills" items={SKILLS} />

//       <Carousel id="languages" title="My Languages" items={LANGUAGES} />
//     </>
//   );
// });

// export const head: DocumentHead = {
//   title: "Siaphix",
//   meta: [
//     {
//       name: "description",
//       content: "Personal website of Siaphix — projects, skills and more.",
//     },
//     { name: "theme-color", content: "#07091a" },
//   ],
// };

import { component$, useSignal, useVisibleTask$ } from '@builder.io/qwik';
// Import the default init function and your named Rust function
import init, { process_data } from '../wasm/map.js';

export default component$(() => {
  const result = useSignal<string>('Initializing WASM module...');

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {
    try {
      await init();
      result.value = process_data("Hello from Qwik!");
    } catch (error) {
      result.value = "Failed to load WASM module.";
      console.error(error);
    }
  });

  return (
    <div style={{ padding: '20px', fontFamily: 'sans-serif' }}>
      <h1>Bun 🥟 + Qwik ⚡ + Rust WASM 🦀</h1>
      <div style={{ padding: '20px', border: '1px solid #ccc', borderRadius: '8px', marginTop: '20px' }}>
        <p><strong>WASM Output:</strong></p>
        <p>{result.value}</p>
      </div>
    </div>
  );
});
