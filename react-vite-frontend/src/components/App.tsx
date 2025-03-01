// Make the rust router use cors to accept FE requests ✅
// Make the rust router return a json response and display it on the FE. ✅
// Use the new 'use' syntax.

/// <reference types="vite/client" />

import React from 'react';
import { useState, useEffect, use, Suspense } from 'react';
import { ErrorBoundary } from 'react-error-boundary';

interface ImportMetaEnv {
  readonly VITE_BACKEND_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

interface ApiResponse {
  message: string;
}

// Your async data fetching function
async function fetchData() {
  const response = await fetch(import.meta.env.VITE_BACKEND_URL);
  if (!response.ok) {
    throw new Error('Failed to fetch data');
  }

  const json = await response.json();

  return json;
}

// The component that uses state and useEffect
const DataComponent: React.FC = () => {
  const [data, setData] = useState<ApiResponse | null>(null);

  useEffect(() => {
    fetchData().then(setData);
  }, []);

  if (!data) return null;

  return (
    <div>
      <h2>{data.message}</h2>
    </div>
  );
};
// Log React version
// The main App component wrapped with Suspense
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
