import { AiTool } from '@/types';
import { Button } from './ui/button';

type AiToolProps = {
  tool: AiTool;
};

export function AiToolComponent({ tool }: AiToolProps) {
  const deleteAiTool = async () => {
    const response = await fetch(
      `${import.meta.env.VITE_AI_TOOLS_URL}/${tool.id}`,
      {
        method: 'DELETE',
      }
    );
    if (!response.ok) {
      throw new Error('Failed to delete tool');
    }
  };

  return (
    <div key={tool.id} className="border p-4 rounded-lg shadow-md">
      <h4 className="text-xl text-gray-200 font-semibold mb-2">
        Name: {tool.name}
      </h4>
      <p className="text-gray-200 mb-2">Description: {tool.description}</p>
      <p className="text-gray-200 mb-2">Company: {tool.company}</p>
      {tool.image_url && (
        <img
          src={tool.image_url}
          alt={tool.name}
          className="w-full h-48 object-cover rounded-md"
        />
      )}
      <Button
        variant={'destructive'}
        className="bg-red-900"
        onClick={deleteAiTool}
      >
        Delete
      </Button>
    </div>
  );
}
