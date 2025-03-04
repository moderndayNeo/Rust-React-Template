import { use } from 'react';
import { AiToolForm } from './AiToolForm';

interface AiTool {
  id: string;
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
  const aiToolsData = use(promise);

  return (
    <div>
      <h3 className="text-2xl font-bold mb-4 text-gray-200">AiTools</h3>
      <AiToolForm />
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {aiToolsData.tools.map((tool: AiTool) => (
          <div key={tool.id} className="border p-4 rounded-lg shadow-md">
            <h4 className="text-xl text-gray-200 font-semibold mb-2">
              Name: {tool.name}
            </h4>
            <p className="text-gray-200 mb-2">Desc.: {tool.description}</p>
            <p className="text-gray-200 mb-2">Company: {tool.company}</p>
            <p className="text-gray-200 font-bold mb-2">
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
