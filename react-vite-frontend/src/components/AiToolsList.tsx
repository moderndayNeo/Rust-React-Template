import { use } from 'react';
import { AiToolComponent } from './AiToolComponent';
import { AiTool } from '@/types';

async function fetchData() {
  const response = await fetch(import.meta.env.VITE_AI_TOOLS_URL);
  if (!response.ok) {
    throw new Error('Failed to fetch data');
  }
  const json = await response.json();
  return json;
}

const promise = fetchData();

export function AiToolsList() {
  const aiToolsData = use(promise);

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {aiToolsData.tools.map((tool: AiTool) => (
        <AiToolComponent tool={tool} />
      ))}
    </div>
  );
}
