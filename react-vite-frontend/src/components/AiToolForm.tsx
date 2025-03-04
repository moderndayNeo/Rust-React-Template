import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { Button } from './ui/button';
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
import { cn } from '@/lib/utils';

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

interface AiToolFormProps {
  className?: string;
  defaultValues?: z.infer<typeof toolSchema>;
  formPurpose: 'create' | 'update';
  idToUpdate?: number;
}

export function AiToolForm({
  className,
  formPurpose,
  defaultValues = {
    name: 'DeepSeek-R6',
    company: 'DeepSeek',
    description: 'Locally-hosted AI Tool',
    image_url: '',
  },
  idToUpdate,
}: AiToolFormProps) {
  const form = useForm<z.infer<typeof toolSchema>>({
    resolver: zodResolver(toolSchema),
    defaultValues,
  });

  function onSubmit(values: z.infer<typeof toolSchema>) {
    console.log(values);

    if (formPurpose === 'create') {
      fetch(import.meta.env.VITE_AI_TOOLS_URL, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(values),
      });
    } else {
      // formPurpose is update
      if (!idToUpdate) {
        throw new Error('idToUpdate is required for update');
      }

      fetch(`${import.meta.env.VITE_AI_TOOLS_URL}/${idToUpdate}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          id: idToUpdate,
          ...values,
        }),
      });
    }
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className={cn(className, 'space-y-8 text-white mb-4')}
      >
        <FormField
          control={form.control}
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
          {formPurpose === 'create' ? 'Submit' : 'Update'}
        </Button>
      </form>
    </Form>
  );
}
