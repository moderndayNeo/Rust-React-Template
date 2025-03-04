import { use } from 'react';
import { Button } from './ui/button';
import { useForm } from 'react-hook-form';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from './ui/form';
import { Input } from './ui/input';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';

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

const toolSchema = z.object({
  name: z
    .string()
    .min(2, {
      message: 'Please enter a tool name',
    })
    .max(50),
  company: z.string().min(2, {
    message: 'Please enter a company name',
  }),
  description: z.string().min(2, {
    message: 'Please enter a description',
  }),
  image_url: z.string().nullable(),
});

export function AiTools() {
  const data = use(promise);
  const form = useForm<z.infer<typeof toolSchema>>({
    resolver: zodResolver(toolSchema),

    defaultValues: {
      name: 'Claude 3.7 Sonnet',
      company: 'Anthropic',
      description: 'Text generation LLM',
      image_url: '',
    },
  });

  function onSubmit(values: z.infer<typeof toolSchema>) {
    console.log(values);

    fetch(import.meta.env.VITE_AI_TOOLS_URL, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(values),
    });
  }

  return (
    <div>
      <h3 className="text-2xl font-bold mb-4 text-gray-200">AiTools</h3>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-8 text-white mb-4"
        >
          <FormField
            control={form.control} // form.control is for managing form state under the hood
            name="name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Tool Name</FormLabel>
                <FormControl>
                  <Input placeholder="Tool Name" {...field} />
                </FormControl>
                <FormDescription>
                  This is the name for your AI Tool.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="company"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Company Name</FormLabel>
                <FormControl>
                  <Input placeholder="Company" {...field} />
                </FormControl>
                <FormDescription>
                  This is the company that produced the AI Tool.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="description"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Description</FormLabel>
                <FormControl>
                  <Input placeholder="Description" {...field} />
                </FormControl>
                <FormDescription>Describe the AI Tool</FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

          <Button type="submit" variant="secondary" className="mt-4">
            Submit
          </Button>
        </form>
      </Form>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {data.tools.map((tool: AiTool) => (
          <div key={tool.name} className="border p-4 rounded-lg shadow-md">
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
