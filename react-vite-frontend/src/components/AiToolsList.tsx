import { use } from 'react';

interface AiTool {
  id: string;
  name: string;
  company: string;
  description: string;
  image_url: string | null;
}

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
        <div key={tool.id} className="border p-4 rounded-lg shadow-md">
          <h4 className="text-xl text-gray-200 font-semibold mb-2">
            Name: {tool.name}
          </h4>
          <p className="text-gray-200 mb-2">Desc.: {tool.description}</p>
          <p className="text-gray-200 mb-2">Company: {tool.company}</p>
          {tool.image_url && (
            <img
              src={tool.image_url}
              alt={tool.name}
              className="w-full h-48 object-cover rounded-md"
            />
          )}
        </div>
      ))}
    </div>
  );
}
