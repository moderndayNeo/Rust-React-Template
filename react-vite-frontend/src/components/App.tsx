// Make the rust router use cors to accept FE requests ✅
// Make the rust router return a json response and display it on the FE. ✅
// Use the new 'use' syntax. ✅

/// <reference types="vite/client" />

import React from 'react';
import { use, Suspense } from 'react';
import { ErrorBoundary } from 'react-error-boundary';

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
    <div>
      <h1>Welcome to the Vite + React + TypeScript App!</h1>
      <ErrorBoundary fallback={<div>Error loading data!</div>}>
        <Suspense fallback={<div>Loading...</div>}>
          <DataComponent />
        </Suspense>
      </ErrorBoundary>
    </div>
  );
};

export default App;
