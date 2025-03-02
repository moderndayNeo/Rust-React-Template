/// <reference types="vite/client" />

import React from 'react';
import { use, Suspense } from 'react';
import { ErrorBoundary } from 'react-error-boundary';
import { AiTools } from './AiTools';

async function fetchData() {
  const response = await fetch(import.meta.env.VITE_BACKEND_URL);
  if (!response.ok) {
    throw new Error('Failed to fetch data');
  }

  const json = await response.json();

  return json;
}

const promise = fetchData();

const DataComponent: React.FC = () => {
  const data = use(promise);

  return (
    <div>
      <h2>{data.message}</h2>
    </div>
  );
};

const App: React.FC = () => {
  return (
    <div className="p-14">
      <h1 className="text-4xl text-blue-700">
        Welcome to the Vite + React + TypeScript App!
      </h1>

      <ErrorBoundary fallback={<div>Error loading data!</div>}>
        <Suspense fallback={<div>Loading...</div>}>
          <DataComponent />
          <AiTools />
        </Suspense>
      </ErrorBoundary>
    </div>
  );
};

export default App;
