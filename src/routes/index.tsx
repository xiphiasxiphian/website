import { component$ } from '@builder.io/qwik';
import type { DocumentHead } from '@builder.io/qwik-city';

import { Carousel } from "./components/carousel/carousel"

import "./styles.css"

interface CarouselItem {
  icon:  string;
  title: string;
  desc:  string;
}

// Data
const PROJECTS: CarouselItem[] = [
  { icon: '🚀', title: 'Project Alpha',   desc: 'A full-stack web application built with Qwik and Node.' },
  { icon: '🎮', title: 'Game Engine',      desc: 'A lightweight 2D game engine written in TypeScript.' },
  { icon: '🤖', title: 'AI Assistant',     desc: 'An LLM-powered productivity tool with custom fine-tuning.' },
  { icon: '📊', title: 'Data Dashboard',   desc: 'Real-time analytics dashboard with interactive charts.' },
  { icon: '🔐', title: 'Auth Library',     desc: 'Zero-dependency authentication library for Node.js.' },
];

const SKILLS: CarouselItem[] = [
  { icon: '⚡', title: 'TypeScript',       desc: 'Strongly typed JavaScript — my go-to for all projects.' },
  { icon: '🌐', title: 'Web Frameworks',   desc: 'Qwik, React, Next.js and the modern frontend ecosystem.' },
  { icon: '🗄️', title: 'Databases',        desc: 'PostgreSQL, MongoDB, Redis and data modelling patterns.' },
  { icon: '☁️', title: 'Cloud & DevOps',   desc: 'AWS, Docker, CI/CD pipelines and infrastructure as code.' },
  { icon: '🎨', title: 'UI/UX Design',     desc: 'Figma, design systems, accessibility and motion design.' },
  { icon: '🧠', title: 'Machine Learning', desc: 'PyTorch, model fine-tuning and deploying ML APIs.' },
];

const LANGUAGES: CarouselItem[] = [
  { icon: '🟦', title: 'TypeScript',  desc: 'Primary language — used daily across frontend and backend.' },
  { icon: '🐍', title: 'Python',      desc: 'Scripting, data science, and ML model development.' },
  { icon: '☕', title: 'Java',        desc: 'Enterprise services and Android application development.' },
  { icon: '🦀', title: 'Rust',        desc: 'Systems programming, performance-critical tools.' },
  { icon: '🐹', title: 'Go',          desc: 'Microservices and high-throughput backend APIs.' },
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
