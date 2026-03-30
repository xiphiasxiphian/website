import { component$, useSignal, $ } from '@builder.io/qwik';
import type { DocumentHead } from '@builder.io/qwik-city';

interface CarouselItem {
  icon:  string;
  title: string;
  desc:  string;
}

interface CarouselSectionProps {
  id:    string;
  title: string;
  items: CarouselItem[];
  visibleCount?: number;  // defaults to 3
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

// Carousel Component
const CarouselSection = component$<CarouselSectionProps>(({
  id,
  title,
  items,
  visibleCount = 3,
}) => {
  const index = useSignal(0); // index of the leftmost visible card
  const maxIndex = items.length - visibleCount;

  const prev$ = $(() => {
    if (index.value > 0) index.value -= 1;
  });

  const next$ = $(() => {
    if (index.value < maxIndex) index.value += 1;
  });

  const goTo$ = $((i: number) => {
    index.value = i;
  });

  // The translate offset: each card is (100 / visibleCount)% wide + gap compensation
  // const translateX = `calc(${index.value} * (100% / ${visibleCount} + (${visibleCount - 1} / ${visibleCount}) * 1rem / ${visibleCount - 1 || 1}))`;

  return (
    <section class={`carousel-section ${id}`} aria-label={title}>
      <h2 class="section-title">{title}</h2>

      <div class="carousel-wrapper">
        {/* Prev button */}
        <button
          class="carousel-btn"
          onClick$={prev$}
          disabled={index.value === 0}
          aria-label="Previous"
        >
          ‹
        </button>

        {/* Track */}
        <div class="carousel-track-outer">
          <div
            class="carousel-track"
            style={{
              transform: `translateX(calc(-${index.value} * (100% / ${visibleCount} + 1rem / ${visibleCount})))`,
            }}
          >
            {items.map((item) => (
              <article key={item.title} class="carousel-card">
                <div class="card-icon" aria-hidden="true">{item.icon}</div>
                <div class="card-title">{item.title}</div>
                <p class="card-desc">{item.desc}</p>
              </article>
            ))}
          </div>
        </div>

        {/* Next button */}
        <button
          class="carousel-btn"
          onClick$={next$}
          disabled={index.value >= maxIndex}
          aria-label="Next"
        >
          ›
        </button>
      </div>

      {/* Dot indicators — one dot per possible stop */}
      {maxIndex > 0 && (
        <div class="carousel-dots" role="tablist" aria-label={`${title} pagination`}>
          {Array.from({ length: maxIndex + 1 }, (_, i) => (
            <button
              key={i}
              role="tab"
              aria-selected={index.value === i}
              aria-label={`Go to slide ${i + 1}`}
              class={`carousel-dot ${index.value === i ? 'active' : ''}`}
              onClick$={() => goTo$(i)}
            />
          ))}
        </div>
      )}
    </section>
  );
});

// ─── Home Page ───────────────────────────────────────────────────────────────
export default component$(() => {
  return (
    <>
      <CarouselSection
        id="projects"
        title="My Projects"
        items={PROJECTS}
      />

      <CarouselSection
        id="skills"
        title="My Skills"
        items={SKILLS}
      />

      <CarouselSection
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
