import { AiToolForm } from './AiToolForm';
import { AiToolsList } from './AiToolsList';

export function AiToolsContainer() {
  return (
    <div>
      <h3 className="text-2xl font-bold mb-4 text-gray-200">AiTools</h3>
      <AiToolForm />
      <AiToolsList />
    </div>
  );
}
