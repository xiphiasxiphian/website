import { component$, Slot } from '@builder.io/qwik';

interface StationProps {
  crs: string;
  name: string;
}

export const StationCard = component$<StationProps>(({ crs, name }) => {
  return (
    <section id={crs} style={{ border: '1px solid #ccc', margin: '20px 0', padding: '20px' }}>
      <header style={{ display: 'flex', justifyContent: 'space-between' }}>
        <h2>{name}</h2>
        <span style={{ fontWeight: 'bold', color: 'gray' }}>{crs}</span>
      </header>

      <div class="station-content">
        <Slot />
      </div>
    </section>
  );
});
