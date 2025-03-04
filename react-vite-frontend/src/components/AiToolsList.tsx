import { use, useState } from 'react';
import { AiToolComponent } from './AiToolComponent';
import { AiTool } from '@/types';
import { Input } from './ui/input';
import { Label } from './ui/label';

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
  const [filterString, setFilterString] = useState('');

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <Label className="text-gray-200">
        Filter by name or company:
        <Input
          value={filterString}
          onChange={(e) => setFilterString(e.target.value)}
        />
      </Label>

      {aiToolsData.tools
        .filter(
          (tool: AiTool) =>
            tool.name.toLowerCase().includes(filterString.toLowerCase()) ||
            tool.company.toLowerCase().includes(filterString.toLowerCase())
        )
        .map((tool: AiTool) => (
          <AiToolComponent tool={tool} key={tool.id} />
        ))}
    </div>
  );
}
