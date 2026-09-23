import { component$, useVisibleTask$, useSignal } from '@builder.io/qwik';

export type StationStatus = boolean | null
export interface Station {
  name: string;
  crs: string;
  lat: number;
  long: number;
  line: string | null;
  status: StationStatus;
}

export const Map = component$(({ stations }: { stations: Station[] }) => {
  const mapRef = useSignal<HTMLElement>();

  useVisibleTask$(async ({ track }) => {
    track(() => stations);

    if (!mapRef.value) { return (<div></div>) }

    const L = (await import('leaflet')).default;
    const map = L.map(mapRef.value).setView([54.0, -2.0], 6);

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 19,
      attribution: '© OpenStreetMap'
    }).addTo(map);

    const getStatusInfo = (status: StationStatus) => {
      switch (status)
      {
        case true: return ["#10b981", "Visited"]
        case false: return ["#f59e0b", "Stopped At"]
        case null: return ["#ef4444", "Not Visited"]
      }
    }

    stations.forEach(station => {
      const [color, tag] = getStatusInfo(station.status);
      const line = station.line ? `Line: ${station.line}<br/>` : ""
      L.circleMarker([station.lat, station.long], {
        radius: 4,
        fillColor: color,
        color: '#000',
        weight: 1,
        opacity: 1,
        fillOpacity: 0.8
      })
        .bindPopup(`<b>${station.name}</b> (${station.crs})<br/>${line}Status: ${tag}`)
      .addTo(map);
    });
  });

  return (
    <div>
      <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
      <div ref={mapRef} style={{ height: '800px', width: '100%', borderRadius: '8px' }} />
    </div>
  );
});
