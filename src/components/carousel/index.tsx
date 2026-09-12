import { component$, $, useSignal } from "@builder.io/qwik";

import "./styles.css";

export interface CarouselItem {
  icon: string;
  title: string;
  desc: string;
}

interface CarouselSectionProps {
  id: string;
  title: string;
  items: CarouselItem[];
  visibleCount?: number; // defaults to 3
}

// Carousel Component
export const Carousel = component$<CarouselSectionProps>(
  ({ id, title, items, visibleCount = 3 }) => {
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
      <section
        class={`carousel-section ${id}`}
        aria-label={title}
        style={{ "--visible-count": visibleCount }}
      >
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
                  <div class="card-icon" aria-hidden="true">
                    {item.icon}
                  </div>
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
          <div
            class="carousel-dots"
            role="tablist"
            aria-label={`${title} pagination`}
          >
            {Array.from({ length: maxIndex + 1 }, (_, i) => (
              <button
                key={i}
                role="tab"
                aria-selected={index.value === i}
                aria-label={`Go to slide ${i + 1}`}
                class={`carousel-dot ${index.value === i ? "active" : ""}`}
                onClick$={() => goTo$(i)}
              />
            ))}
          </div>
        )}
      </section>
    );
  },
);
