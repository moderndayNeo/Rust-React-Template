import React, { use } from 'react';

interface AiTool {
  name: string;
  company: string;
  description: string;
  image: string | null;
  monthly_price_usd: number;
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

export function AiTools() {
  const data = use(promise);

  console.log(data);

  return (
    <div className="p-4">
      <h3 className="text-2xl font-bold mb-4">AiTools</h3>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {data.tools.map((tool: AiTool) => (
          <div key={tool.name} className="border p-4 rounded-lg shadow-md">
            <h4 className="text-xl font-semibold mb-2">{tool.name}</h4>
            <p className="text-gray-700 mb-2">{tool.description}</p>
            <p className="text-gray-500 mb-2">{tool.company}</p>
            <p className="text-gray-900 font-bold mb-2">
              ${tool.monthly_price_usd}/month
            </p>
            {tool.image && (
              <img
                src={tool.image}
                alt={tool.name}
                className="w-full h-48 object-cover rounded-md"
              />
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
