import { component$ } from '@builder.io/qwik';
import { routeLoader$ } from '@builder.io/qwik-city';
import Papa from 'papaparse';
import * as fs from 'fs/promises';
import { Map, type StationStatus, type Station } from '~/components/map';
import toml from 'toml';

export const useStationsData = routeLoader$<Station[]>(async () => {
  try
  {
    const res = await fetch("https://raw.githubusercontent.com/davwheat/uk-railway-stations/main/stations.csv");
    const csvText = await res.text();
    const parsed = Papa.parse(csvText, { header: true, skipEmptyLines: true });

    let visitedConfig: Record<string, StationStatus> = {}
    try
    {
      const filePath = "./src/routes/railways/visited.toml";
      const file = await fs.readFile(filePath, "utf-8");
      visitedConfig = toml.parse(file);
    }
    catch (e)
    {
      console.warn("visited.toml not found, defaulting to all not visited - error: ", e);
    }

    const stations = parsed.data
      .map((row: any) => (
        {
          name: row.stationName,
          crs: row.crsCode,
          lat: parseFloat(row.lat),
          long: parseFloat(row.long),
          status: visitedConfig[row.crsCode] ?? null,
        }
      ))
      .filter(s => !isNaN(s.lat) && !isNaN(s.long))

    return stations;
  }
  catch (e)
  {
    console.error("Error loading stations: ", e);
    return []
  }
})


export default component$(() => {
  const stationsSignal = useStationsData();

  return (
    <main style={{ padding: '20px', fontFamily: 'sans-serif' }}>
      <h1>Stations Tracker</h1>
      <Map stations={stationsSignal.value} />
    </main>
  );
})
