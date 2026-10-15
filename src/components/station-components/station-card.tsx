import { component$, Slot } from "@builder.io/qwik";
import "./styles.css";

interface StationProps {
  crs: string;
  name: string;
}

export const StationCard = component$<StationProps>(({ crs, name }) => {
  return (
    <section id={crs} class="station-card" aria-labelledby={`${crs}-title`}>
      <header class="station-header">
        <h2 id={`${crs}-title`} class="station-name">
          {name}
        </h2>
        <span class="station-crs">{crs}</span>
      </header>

      <div class="station-content">
        <Slot />
      </div>
    </section>
  );
});
