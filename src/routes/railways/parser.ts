import { readFile, readdir } from "fs/promises";
import { join, parse } from "path";
import { parse as parseToml } from 'smol-toml';

const EXTENSION: string = ".toml";

export interface StationEntry
{
  crs: string,
  line: string,
  status: boolean,
};

function extractEntries(
  node: Record<string, any>,
  currentLine: string,
): StationEntry[]
{
  const results: StationEntry[] = [];

  for (const [key, value] of Object.entries(node))
  {
    if (typeof value === "boolean")
    {
      const entry: StationEntry = {
        crs: key,
        line: currentLine,
        status: value
      };

      results.push(entry);
    }
    else if (typeof value === "object" && value !== null && !Array.isArray(value))
    {
      const nextLine = `${currentLine}/${key}`;
      results.push(...extractEntries(value, nextLine))
    }
  }

  return results;
}

async function processFile(
  path: string,
  filename: string,
): Promise<StationEntry[]>
{
  try
  {
    const content = await readFile(path, "utf-8");
    const data = parseToml(content);
    const results: StationEntry[] = [];

    results.push(...extractEntries(data, filename));

    return results;
  }
  catch (e)
  {
    console.error(`Failed to parse file at ${path}: ${e}`);
    return [];
  }
}

export async function getStationEntries(
  path: string
): Promise<StationEntry[]>
{
  const dirEntries = await readdir(path, {
    recursive: true,
    withFileTypes: true
  });

  const files = dirEntries.filter((entry) => entry.isFile() && entry.name.endsWith(EXTENSION));

  const promises = files.map((entry) => {
    const directory = entry.parentPath;
    const fullPath = join(directory, entry.name);
    const baseName = parse(entry.name).name;

    return processFile(fullPath, baseName);
  });

  const nestedResults = await Promise.all(promises);
  return nestedResults.flat()
}
