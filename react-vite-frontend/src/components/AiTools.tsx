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
    <div>
      <h3>AiTools</h3>
      <div>
        {data.tools.map((tool: AiTool) => (
          <div key={tool.name}>
            <h4>{tool.name}</h4>
            <p>{tool.description}</p>
            <p>{tool.company}</p>
            <p>{tool.monthly_price_usd}</p>
            {tool.image && <img src={tool.image} alt={tool.name} />}
          </div>
        ))}
      </div>
    </div>
  );
}
