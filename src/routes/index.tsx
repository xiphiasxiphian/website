import { component$ } from '@builder.io/qwik';
import type { DocumentHead } from '@builder.io/qwik-city';

import "./styles.css"
import { Carousel, CarouselItem } from '~/components/carousel';

// Data
const PROJECTS: CarouselItem[] = [
  { icon: '🚀', title: 'Test 1',   desc: 'Description 1' },
  { icon: '🎮', title: 'Test 2',   desc: 'Description 2' },
  { icon: '🤖', title: 'Test 3',   desc: 'Description 3' },
  { icon: '📊', title: 'Test 4',   desc: 'Description 4' },
  { icon: '🔐', title: 'Test 5',   desc: 'Description 5' },
];

const SKILLS: CarouselItem[] = [
  { icon: '⚡', title: 'Test 1',   desc: 'Description 1' },
  { icon: '🌐', title: 'Test 2',   desc: 'Description 2' },
  { icon: '🗄️', title: 'Test 3',   desc: 'Description 3' },
  { icon: '☁️', title: 'Test 4',   desc: 'Description 4' },
  { icon: '🎨', title: 'Test 5',   desc: 'Description 5' },
  { icon: '🧠', title: 'Test 6',   desc: 'Description 6' },
];

const LANGUAGES: CarouselItem[] = [
  { icon: '🟦', title: 'Test 1',   desc: 'Description 1' },
  { icon: '🐍', title: 'Test 2',   desc: 'Description 2' },
  { icon: '☕', title: 'Test 3',   desc: 'Description 3' },
  { icon: '🦀', title: 'Test 4',   desc: 'Description 4' },
  { icon: '🐹', title: 'Test 5',   desc: 'Description 5' },
];


export default component$(() => {
  return (
    <>
      <Carousel
        id="projects"
        title="My Projects"
        items={PROJECTS}
      />

      <Carousel
        id="skills"
        title="My Skills"
        items={SKILLS}
      />

      <Carousel
        id="languages"
        title="My Languages"
        items={LANGUAGES}
      />
    </>
  );
});

// ─── Document Head ───────────────────────────────────────────────────────────
export const head: DocumentHead = {
  title: 'Siaphix',
  meta: [
    { name: 'description', content: 'Personal website of Siaphix — projects, skills and more.' },
    { name: 'theme-color', content: '#07091a' },
  ],
};
