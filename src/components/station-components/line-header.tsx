import { component$, Slot } from "@builder.io/qwik";
import "./styles.css";

interface LineStat {
  label: string;
  value: string;
}

interface LineHeaderProps {
  title: string;
  eyebrow?: string;
  stats?: LineStat[];
}

export const LineHeader = component$<LineHeaderProps>(
  ({ title, eyebrow, stats }) => {
    return (
      <header class="line-header">
        {eyebrow && <span class="line-eyebrow">{eyebrow}</span>}

        <h1 class="line-title">{title}</h1>

        <div class="line-summary">
          <Slot />
        </div>

        {stats && stats.length > 0 && (
          <dl class="line-stats">
            {stats.map(({ label, value }) => (
              <div key={label} class="line-stat">
                <dt>{label}</dt>
                <dd>{value}</dd>
              </div>
            ))}
          </dl>
        )}

        <div class="line-track" aria-hidden="true" />
      </header>
    );
  },
);
